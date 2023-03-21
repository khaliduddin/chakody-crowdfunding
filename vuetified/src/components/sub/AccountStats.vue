<script setup>
const props = defineProps(['isSignedIn', 'wallet'])

const connectWallet = () => {
    // console.log(CONTRACT_NAME)
    props.wallet.signIn()
}

</script>
<template>
    <v-expansion-panels
        multiple
    >                                    
        <v-expansion-panel>
            <v-expansion-panel-title>
                <label>Investor Info</label>
                <v-btn>Test</v-btn>
            </v-expansion-panel-title>
            <v-expansion-panel-text>Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, ratione debitis quis est labore voluptatibus! Eaque cupiditate minima</v-expansion-panel-text>
        </v-expansion-panel>
    </v-expansion-panels>               
    
    <section>
        <h4></h4>
        <v-btn :class="props.isSignedIn ? 'd-none' : 'd-block'" @click="connectWallet">Connect NEAR Wallet</v-btn>
        <v-btn :class="!props.isSignedIn ? 'd-none' : 'd-block'">Disconnect Wallet</v-btn>
        <label>Connected to Near Wallet</label>
        <label>Wallet Balance</label>
        <label>Total Deposit</label>
        <v-btn>Claim</v-btn>
    </section>
</template>