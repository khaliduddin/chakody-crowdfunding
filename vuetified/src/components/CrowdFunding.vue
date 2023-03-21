<script setup>
import { onMounted, ref } from 'vue';
import { Contract } from '../connectors/near-interface'
import { Wallet } from '../connectors/near-wallet'
import { CONTRACT_NAME } from '../config/app-settings'
import ProjectInfo from './sub/ProjectInfo.vue'
import AccountStats from './sub/AccountStats.vue';

// When creating the wallet you can choose to create an access key, so the user
// can skip signing non-payable methods when interacting with the contract
const wallet = new Wallet({ createAccessKeyFor: CONTRACT_NAME })

// Abstract the logic of interacting with the contract to simplify your project
const contract = new Contract({ contractId: CONTRACT_NAME, walletToUse: wallet });

let targetAmount = ref(0)
let isSignedIn = ref(false)
let isMounted = ref(false)

onMounted(async () => {
    console.log('mounted')
    isSignedIn.value = await wallet.startUp();

    if (isSignedIn){
        signedInFlow()
    }else{
        signedOutFlow()
    }

    // fetchBeneficiary()
    await getAndShowDonations()
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

async function getAndShowDonations(){
//   document.getElementById('donations-table').innerHTML = 'Loading ...'

  // Load last 10 donations
  let donations = await contract.latestDonations()
  console.log(donations)

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

const desserts= [
          {
            name: 'Frozen Yogurt',
            calories: 159,
          },
          {
            name: 'Ice cream sandwich',
            calories: 237,
          },
          {
            name: 'Eclair',
            calories: 262,
          },
          {
            name: 'Cupcake',
            calories: 305,
          }
        ]

const depositFunds = () => {
    console.log('deposit funds')
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
                </div>

                <label v-else>Loading... Please Wait!! </label>                
               
                <section>
                    <v-row>
                        <v-col>
                            <v-table>
                                <thead>
                                <tr>
                                    <th class="text-left">
                                    Investor
                                    </th>
                                    <th class="text-left">
                                    Total Fund Deposited Ⓝ
                                    </th>
                                </tr>
                                </thead>
                                <tbody>
                                <tr
                                    v-for="item in desserts"
                                    :key="item.name"
                                >
                                    <td>{{ item.name }}</td>
                                    <td>{{ item.calories }}</td>
                                </tr>
                                </tbody>
                            </v-table>
                        </v-col>
                        <v-col>
                            <v-card variant="outlined">                                
                                <v-card-item>
                                    <v-card-title>Deposit your Funds</v-card-title>
                                    <!-- <v-card-subtitle>TEST</v-card-subtitle> -->
                                </v-card-item>
                                <v-row>
                                    <v-btn>
                                        <v-chip color="primary" variant="elevated">$50</v-chip>
                                    </v-btn>
                                    <v-btn>
                                        <v-chip color="primary" variant="elevated">$100</v-chip>
                                    </v-btn>
                                    <v-btn>
                                        <v-chip color="primary" variant="elevated">$250</v-chip>
                                    </v-btn>
                                    <v-btn>
                                        <v-chip color="primary" variant="elevated">$500</v-chip>
                                    </v-btn>
                                </v-row>
                                <v-card-text>Fund deposit amount (in Ⓝ)</v-card-text>
                                <v-card-text>
                                    <v-row>
                                        <v-text-field
                                            density="compact"
                                            variant="solo"
                                            label=""
                                            single-line
                                            hide-details                                                                                      
                                        ></v-text-field>
                                        <v-chip label>Ⓝ</v-chip>
                                        <v-btn color="primary" @click="depositFunds">Deposit Fund</v-btn>
                                    </v-row>
                                </v-card-text>
                            </v-card>
                        </v-col>
                    </v-row>
                </section>
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