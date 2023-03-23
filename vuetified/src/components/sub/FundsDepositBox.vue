<script setup>
const depositAmounts = [
    100, 250, 500, 1000
]

const depositFunds = () => {
    console.log('deposit funds')
}

const clearInput = () => {}
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
                <v-btn v-for="num in depositAmounts" class="ma-2" rounded="pill" color="primary">${{ num }}</v-btn>
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
                >
                </v-text-field>
                <v-text-field 
                    density="compact" 
                    variant="filled" 
                    label="Ⓝ NEAR"
                    class="mr-4 w-25"
                >
                </v-text-field>           
                <v-btn class="pa-1 ma-2 bg-red-lighten-1" size="small" @click="clearInput">
                    <v-icon
                    icon="mdi-refresh"
                    size="large"                    
                    />
                </v-btn>     
            </v-row>
            <v-btn class="mt-6" color="primary" @click="depositFunds">Deposit Fund</v-btn>
        </v-card-text>
    </v-card>
</template>