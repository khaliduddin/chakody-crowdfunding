<script setup>
import { onMounted, ref } from 'vue';
import { Contract } from '../connectors/near-interface'
import { Wallet } from '../connectors/near-wallet'
import { CONTRACT_NAME } from '../config/app-settings'
import ProjectInfo from './sub/ProjectInfo.vue'
import AccountStats from './sub/AccountStats.vue'
import FundStats from './sub/FundStats.vue'
import FundsDepositBox from './sub/FundsDepositBox.vue'
import { findNear2UsdPrice, roundToTwoDecimals } from '@/connectors/common';

// When creating the wallet you can choose to create an access key, so the user
// can skip signing non-payable methods when interacting with the contract
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_NAME })

// Abstract the logic of interacting with the contract to simplify your project
const contract = new Contract({ contractId: CONTRACT_NAME, walletToUse: wallet });

let targetAmount = ref(0)
let isSignedIn = ref(false)
let isMounted = ref(false)
let deposits = ref([])
let closingDate = ref(0)
let totalDeposits = ref({
    nearAmount: 0,
    usdAmount: 0
})
let walletAccountDeposit = ref(0)
let targetAmountInNear = ref(0)
let projectBeneficiary = ref('')

const setNearAmount = async (amount) => {    
    const near2usd = await findNear2UsdPrice()
    const amount_in_near = amount / near2usd
    const rounded_two_decimals = Math.round(amount_in_near * 100) / 100
    targetAmountInNear.value = rounded_two_decimals
}

onMounted(async () => {
    console.log('mounted')
    isSignedIn.value = await wallet.startUp();

    if (isSignedIn){
        signedInFlow()
    }else{
        signedOutFlow()
    }

    projectBeneficiary.value = await fetchBeneficiary()
    await getAndShowDeposits()
    await getAndShowContractData()
    await setNearAmount(targetAmount.value)

    isMounted.value = !isMounted.value
})

// Display the signed-out-flow container
function signedOutFlow() {
    isSignedIn.value = !isSignedIn.value
    console.log('sign out')
//   document.querySelector('.signed-out-flow').style.display = 'block'
}

async function signedInFlow() {
  // Displaying the signed in flow container
  // document.querySelectorAll('.signed-in-flow').forEach(elem => elem.style.display = 'block')

  // Check if there is a transaction hash in the URL
  const urlParams = new URLSearchParams(window.location.search);
  const txhash = urlParams.get("transactionHashes")

  if(txhash !== null){
    // Get result from the transaction
    let result = await contract.getDonationFromTransaction(txhash)
    console.log(result)    
  }

}

async function getAndShowDeposits(){
//   document.getElementById('donations-table').innerHTML = 'Loading ...'
  
  deposits.value = await contract.latestDeposits()
  console.log(deposits.value)

  deposits.value.forEach(element => {
    console.log(`deposit of ${element.account_id} is ${element.total_amount} `, )
    totalDeposits.value.nearAmount += parseFloat(element.total_amount)
    if(element.account_id === wallet.accountId) {
        walletAccountDeposit = element.total_amount
    }    

    console.log('***********', element.account_id === wallet.accountId ? element.total_amount : 0)
  });

  totalDeposits.value.nearAmount = roundToTwoDecimals(totalDeposits.value.nearAmount)
  totalDeposits.value.usdAmount = await getUsdFromNear(totalDeposits.value.nearAmount)
}

const getAndShowContractData = async () => {
  
  console.log('contract data block')

  targetAmount.value = await contract.getTargetAmount()
  console.log('target', targetAmount.value)

  let deadline = await contract.getFundingDeadline()
//   closingDate.value = new Date(deadline/1000000).toLocaleDateString("en-US")
  closingDate.value = new Date(deadline/1000000)
  closingDate.value.setDate(closingDate.value.getDate() + 90)
  closingDate.value = closingDate.value.toLocaleDateString("en-US")  
}

const fetchBeneficiary = async () => {
    return await contract.getBeneficiary()
}

const getUsdFromNear = async (amount_in_near) => {
    const near2usd = await findNear2UsdPrice()
    const amount_in_usd = amount_in_near * near2usd
    const rounded_two_decimals = roundToTwoDecimals(amount_in_usd)
    return rounded_two_decimals
}
</script>

<template>
    <v-container class="fill-height">        
        <v-responsive class="d-flex align-center text-center fill-height">
            <v-sheet class="mx-auto my-0 pa-8" color="transparent">
                <h2 class="pb-6 text-primary">CHAKODY L.L.C | CrowdFunding for Pre-seed round</h2>  
                
                <div v-if="isMounted">
                    <ProjectInfo 
                        :targetAmount="targetAmount" 
                        :targetAmountInNear="targetAmountInNear"
                        :deadline="closingDate"
                        :totalDeposits = "totalDeposits" />                
                    <br />
                    <AccountStats 
                        :isSignedIn="isSignedIn" 
                        :wallet="wallet" 
                        :walletAccountDeposit="walletAccountDeposit" />     
                    <br />     
                    <v-row>
                        <v-col>
                            <FundStats :deposits="deposits" />
                        </v-col>
                        <v-col cols="6">
                            <FundsDepositBox :contract="contract" />
                        </v-col>
                    </v-row>             
                </div>

                <label v-else>Loading... Please Wait!! </label>    
                
                <v-sheet class="pa-4 my-4 bg-grey-lighten-1" v-if="projectBeneficiary === wallet.accountId">
                    <v-label>Beneficiary Activity</v-label>
                    <v-label>{{ projectBeneficiary }}</v-label>
                    <br />
                    <v-btn>Claim</v-btn>
                    <v-spacer />
                    <v-btn>Change Deadline</v-btn>
                </v-sheet>

                <v-sheet class="pa-4 my-4 bg-grey-darken-1">
                    Development Data
                </v-sheet>               
            </v-sheet>            
        </v-responsive>
    </v-container>
</template>