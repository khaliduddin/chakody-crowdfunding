<script setup>
const depositFunds = () => {
    console.log('deposit funds')
}
</script>
<template>
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
</template>