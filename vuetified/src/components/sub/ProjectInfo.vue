<script setup>
const props = defineProps(['targetAmount'])
const targetAmount = props.targetAmount
console.log(props.targetAmount)
</script>
<template>
    <v-card>
        <v-card-title class="bg-orange-lighten-3 text-left">Funding Information</v-card-title>
        <v-card-text class="pa-4">
            <v-row justify="space-between">
                <v-col class="text-left font-weight-bold">
                    <label>Target Fund Amount (in USD)</label>
                    <label class="ml-2 text-h6 text-decoration-underline">${{ targetAmount }}</label>
                </v-col>                            
                <v-col class="text-right font-weight-bold">
                    <label>Crowdfunding Deadline</label>
                    <label class="ml-2 text-h6 text-decoration-underline">Date</label>
                </v-col>
            </v-row>
            <v-row>
                <v-col class="text-left font-weight-bold">                                
                    <label>Received Total Funds</label>
                    <label class="ml-2 text-h6 text-decoration-underline">$0</label>
                </v-col>                            
                <v-col class="text-right">                    
                    <v-btn rounded="pill" size="small" color="info">Project Info</v-btn>
                    <v-btn class="ml-2" rounded="pill" size="small" color="primary">FAQ</v-btn>
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>