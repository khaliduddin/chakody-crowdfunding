<script setup>
import { onMounted, ref } from 'vue';
import { Contract } from '../connectors/near-interface'
import { Wallet } from '../connectors/near-wallet'
import { CONTRACT_NAME } from '../config/app-settings'
import ProjectInfo from './sub/ProjectInfo.vue'
import AccountStats from './sub/AccountStats.vue'
import FundStats from './sub/FundStats.vue'
import FundsDepositBox from './sub/FundsDepositBox.vue'

// When creating the wallet you can choose to create an access key, so the user
// can skip signing non-payable methods when interacting with the contract
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_NAME })

// Abstract the logic of interacting with the contract to simplify your project
const contract = new Contract({ contractId: CONTRACT_NAME, walletToUse: wallet });

let targetAmount = ref(0)
let isSignedIn = ref(false)
let isMounted = ref(false)
let deposits = ref([])

onMounted(async () => {
    console.log('mounted')
    isSignedIn.value = await wallet.startUp();

    if (isSignedIn){
        signedInFlow()
    }else{
        signedOutFlow()
    }

    // fetchBeneficiary()
    await getAndShowDeposits()
    await getAndShowContractData()

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
    // document.querySelector('[data-behavior=donation-so-far]').innerText = result

    // show notification
    // document.querySelector('[data-behavior=notification]').style.display = 'block'

    // remove notification again after css animation completes
    // setTimeout(() => {
    //   document.querySelector('[data-behavior=notification]').style.display = 'none'
    // }, 11000)
  }

}

async function getAndShowDeposits(){
//   document.getElementById('donations-table').innerHTML = 'Loading ...'

  // Load last 10 donations
  deposits.value = await contract.latestDeposits()
  console.log(deposits.value)

//   document.getElementById('donations-table').innerHTML = ''

//   donations.forEach(elem => {
//     let tr = document.createElement('tr')
//     tr.innerHTML = `
//       <tr>
//         <th scope="row">${elem.account_id}</th>
//         <td>${elem.total_amount}</td>
//       </tr>
//     `
//     document.getElementById('donations-table').appendChild(tr)
//   })
}

// const connectWallet = () => {
//     wallet.signIn()
// }

const getAndShowContractData = async () => {
  // Load last 10 donations
  // let contractData = await contract.contractData()

  console.log('contract data block')

  targetAmount.value = await contract.getTargetAmount()
  console.log('target', targetAmount.value)

  // let deadline = await contract.getFundingDeadline()

  // var date2 = new Date(deadline/1000000) // 1678991332821693200 nanoseconds to milliseconds
  // console.log('yo1', date2)
  // console.log('yo2', date2.toString())
//   document.getElementById('funding_deadline').innerHTML = new Date(date2.setDate(date2.getDate() + 90))
}
</script>

<template>
    <v-container class="fill-height">        
        <v-responsive class="d-flex align-center text-center fill-height">
            <v-sheet class="mx-auto my-0 pa-8" color="transparent">
                <h2 class="pb-6 text-primary">CHAKODY L.L.C | CrowdFunding for Pre-seed round</h2>  
                
                <div v-if="isMounted">
                    <ProjectInfo :targetAmount="targetAmount" />                
                    <br />
                    <AccountStats :isSignedIn="isSignedIn" :wallet="wallet" />     
                    <br />     
                    <v-row>
                        <v-col>
                            <FundStats :deposits="deposits" />
                        </v-col>
                        <v-col>
                            <FundsDepositBox />
                        </v-col>
                    </v-row>             
                </div>

                <label v-else>Loading... Please Wait!! </label>                
               
                <section>
                    Beneficiary Activity
                    <v-btn>Claim</v-btn>
                    <v-btn>Change Deadline</v-btn>
                </section>
                <section>
                    Development Data
                </section>
            </v-sheet>
            <!-- <v-btn
                href="https://chakody.com/"
                min-width="164"
                target="_blank"
                variant="text"
            >
                <v-icon
                icon="mdi-view-dashboard"
                size="large"
                start
                />

                Responsive Components
            </v-btn> -->
        </v-responsive>
    </v-container>
</template>