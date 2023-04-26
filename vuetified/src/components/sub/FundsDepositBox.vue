<script setup>
import { ref } from 'vue'

const props = defineProps(['contract', 'closingDate'])
const CLEAR = ''
const MIN_DEPOSIT = 1

let txnInProgress = false

let usdAmount = ref('')
let usd2near = ref('')
let customUsd = ref('')
let depositMessage = ref('')
let depositBtnText = ref('Deposit Fund')

const depositAmounts = [
    100, 250, 500, 1000
]

const depositFunds = async () => {    
    console.log('deposit funds')
    try {
        if(validDeposit() && !txnInProgress) {
            txnInProgress = true
            depositBtnText.value = 'Deposit in Progress.. Please wait'
            await props.contract.deposit(usd2near.value)
            depositMessage.value = 'Deposit is successful'
            // props.contract.getAndShowDeposits()       
            txnInProgress = !txnInProgress     
            depositBtnText.value = 'Deposit Fund'
        } else {
            depositMessage.value = 'Deposit amount is required a Minimum $100'
        }                
    } catch (error) {
        txnInProgress = false
        depositMessage.value = 'Something went wrong! Refresh page, signin and try again.'
        throw error
    }
}

const validDeposit = () => {
    // if (
    //         usd2near.value >= MIN_DEPOSIT 
    //         // && usd2near.value >= walletBalance
    //     ) {
    //     // txnInProgress = true
    //     return true
    // } else {
    //     return false
    // }
    return usd2near.value >= MIN_DEPOSIT ? true: false
         
}

const setNearAmount = async (amount) => {
    const API_NEAR_USD = "https://api.coingecko.com/api/v3/simple/price?ids=near&vs_currencies=usd"
    depositMessage.value = '' // reset message
    let data = await fetch(API_NEAR_USD).then(response => response.json())
    const near2usd = data['near']['usd']
    const amount_in_near = amount / near2usd
    const rounded_two_decimals = Math.round(amount_in_near * 100) / 100
    usdAmount.value = amount
    usd2near.value = rounded_two_decimals
}

const clearInput = () => {
    usdAmount.value = CLEAR
    usd2near.value = CLEAR
    customUsd.value = CLEAR
    depositMessage.value = CLEAR
}
</script>
<template>
    <v-card variant="outlined" class="pa-2">                                
        <v-card-item>
            <v-card-title class="py-3">Deposit Funds</v-card-title>
            <!-- <v-card-subtitle>TEST</v-card-subtitle> -->
        </v-card-item>
        <v-card-text>
            <v-row class="py-2">  
                <v-spacer></v-spacer>          
                <v-btn v-for="num in depositAmounts" @click="setNearAmount(num)" class="ma-2" rounded="pill" color="primary">${{ num }}</v-btn>
                <v-spacer></v-spacer>          
            </v-row>
            <v-row class="pa-4">
                <v-spacer></v-spacer> 
                <v-label class="text-grey-darken-8 font-weight-bold">Other amount</v-label>
                <v-spacer></v-spacer> 
                <v-text-field 
                    density="compact" 
                    variant="underlined" 
                    label="$ USD"
                    class="mr-4"
                    style="width: 70px"
                    v-model="customUsd"
                >
                </v-text-field>
                <v-btn 
                    class="pa-1 ma-2 bg-green-lighten-1" 
                    size="small" 
                    @click="setNearAmount(customUsd)" 
                    :disabled="(customUsd == CLEAR)? 'disabled': false"
                >
                    <v-icon
                    icon="mdi-check"
                    size="large"                    
                    />
                </v-btn>                 
                <v-spacer></v-spacer> 
            </v-row>            
        </v-card-text>
        <v-card-text>Fund deposit amount (Ⓝ NEAR) to invest</v-card-text>
        <v-card-text>
            <v-row>
                <v-text-field 
                    density="compact" 
                    variant="filled" 
                    label="$ USD"
                    class="mr-4 w-25"
                    v-model="usdAmount"
                >
                </v-text-field>
                <v-text-field 
                    density="compact" 
                    variant="filled" 
                    label="Ⓝ NEAR"
                    class="mr-4 w-25"
                    v-model="usd2near"
                >
                </v-text-field>           
                <v-btn class="pa-1 ma-2 bg-red-lighten-1" size="small" @click="clearInput">
                    <v-icon
                    icon="mdi-refresh"
                    size="large"                    
                    />
                </v-btn>     
            </v-row>
            <v-label v-if="props.closingDate < new Date().toLocaleDateString('en-US')">Crowdfunding is closed</v-label>
            <v-btn v-else class="mt-6 mb-6" color="primary" @click="depositFunds" :disabled="(usd2near == CLEAR)? 'disabled': false">
                {{ depositBtnText }}
            </v-btn>
            <v-row>                
                <v-label class="d-flex flex-wrap text-error font-weight-bold">{{ depositMessage }}</v-label>
            </v-row>
        </v-card-text>
    </v-card>
</template>