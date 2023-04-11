<script setup>
const props = defineProps(['isSignedIn', 'wallet', 'walletAccountDeposit'])

const connectWallet = () => {
    props.wallet.signIn()
}

const disconnectWallet = () => {
    props.wallet.signOut()
}

const test = async () => {
    console.log('wallet obj ', props.wallet)
    console.log(props.wallet.accountId)
    // console.log(await props.wallet.walletSelector.store.getState())
    console.log(props.walletAccountDeposit)
}

</script>
<template>
    <v-expansion-panels
        multiple
    >                                    
        <v-expansion-panel>
            <v-expansion-panel-title class="bg-orange-lighten-3">
                <label class="text-h6">Your Account Info</label>
                <label class="mx-2">{{ props.wallet.accountId }}</label>
                <v-spacer></v-spacer>
                <v-btn rounded="pill" size="small" color="info" :class="props.isSignedIn ? 'd-none' : 'd-block mr-6'" @click="connectWallet">Connect NEAR Wallet</v-btn>
                <v-btn rounded="pill" size="small" color="red-lighten-1" :class="!props.isSignedIn ? 'd-none' : 'd-block mr-6'" @click="disconnectWallet">Disconnect Wallet</v-btn>
            </v-expansion-panel-title>
            <v-expansion-panel-text>
                <label v-if="!props.isSignedIn">Wallet is not connected</label>
                <div v-else>
                    <v-row>
                        <v-col class="text-left font-weight-bold mt-3">
                            <label class="mr-4">Funds Deposited (in Total)</label>
                            <label>Ⓝ {{ props.walletAccountDeposit }}</label>
                        </v-col>
                        <v-col class="text-left font-weight-bold mt-3">
                            <!-- <v-text-field density="compact" variant="outlined" class="mr-4 w-25"></v-text-field> -->
                            <v-btn color="info">Claim</v-btn>
                        </v-col>   
                    </v-row>
                    <v-row>
                        <!-- <v-col class="text-left font-weight-bold">
                            <v-btn @click="test">Test</v-btn>
                        </v-col> -->
                        <v-col class="text-left font-weight-bold">
                            <v-label>Note: Claims will enabled after funding deadline only when funding target is not reached</v-label>
                        </v-col>                        
                    </v-row>
                </div>
            </v-expansion-panel-text>
        </v-expansion-panel>
    </v-expansion-panels>
</template>