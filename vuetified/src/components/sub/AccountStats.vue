<script setup>
const props = defineProps(['isSignedIn', 'wallet'])

const connectWallet = () => {
    props.wallet.signIn()
}

const disconnectWallet = () => {
    props.wallet.signOut()
}


</script>
<template>
    <v-expansion-panels
        multiple
    >                                    
        <v-expansion-panel>
            <v-expansion-panel-title class="bg-orange-lighten-3">
                <label class="text-h6">Your Account Info</label>
                <v-spacer></v-spacer>
                <v-btn rounded="pill" size="small" color="info" :class="props.isSignedIn ? 'd-none' : 'd-block mr-6'" @click="connectWallet">Connect NEAR Wallet</v-btn>
                <v-btn rounded="pill" size="small" color="red-lighten-1" :class="!props.isSignedIn ? 'd-none' : 'd-block mr-6'" @click="disconnectWallet">Disconnect Wallet</v-btn>
            </v-expansion-panel-title>
            <v-expansion-panel-text>
                <label v-if="!props.isSignedIn">Wallet is not connected</label>
                <div v-else>
                    <label class="font-weight-bold">Wallet Balance</label>
                    <label>Total Deposit</label>
                    <v-btn>Claim</v-btn>
                </div>
            </v-expansion-panel-text>
        </v-expansion-panel>
    </v-expansion-panels>
</template>