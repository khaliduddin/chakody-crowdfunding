<script setup>
import ProjectInfoDialog from './ProjectInfoDialog.vue';

// import { findNear2UsdPrice } from '@/connectors/common'
const props = defineProps(['targetAmount', 'deadline', 'totalDeposits', 'targetAmountInNear'])
const targetAmount = props.targetAmount
// console.log(props.targetAmount)
// console.log(await findNear2UsdPrice())
</script>
<template>
    <v-card>
        <v-card-title class="bg-orange-lighten-3 text-left">Funding Information</v-card-title>
        <v-card-text class="pa-4">
            <v-row justify="space-between">
                <v-col class="text-left font-weight-bold">
                    <label>Target Fund Amount (in USD)</label>
                    <label class="ml-2 text-h6 text-decoration-underline">${{ targetAmount }}</label>
                    <label> [Ⓝ {{ props.targetAmountInNear }}]</label>
                </v-col>                            
                <v-col class="text-right font-weight-bold">
                    <label>Crowdfunding Deadline</label>
                    <label class="ml-2 text-h6 text-decoration-underline">{{ props.deadline }}</label>
                </v-col>
            </v-row>
            <v-row>
                <v-col class="text-left font-weight-bold">                                
                    <label>Received Total Funds</label>
                    <label class="ml-2 text-h6 text-decoration-underline">
                        ${{ props.totalDeposits.usdAmount }}                        
                    </label>
                    <label> [Ⓝ {{ props.totalDeposits.nearAmount }}]</label>
                </v-col>                            
                <v-col class="text-right">                    
                    <!-- <v-btn rounded="pill" size="small" color="info">Project Info</v-btn> -->
                    <!-- <v-btn class="ml-2" rounded="pill" size="small" color="primary">FAQ</v-btn> -->
                    <project-info-dialog />
                </v-col>
            </v-row>
        </v-card-text>
    </v-card>
</template>