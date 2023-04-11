<script setup>
import { ref } from 'vue'

let dialog = ref(false)
const projectInfo = [
    {
        id: '1',
        description: 'CHAKODY LLC is a tech company operating on Blockchain, Metaverse based products and services'
    },
    {
        id: '2',
        description: 'Crowdfunding project primary goal is to build our team through this pre-seed round'
    },
    {
        id: '3',
        description: 'Investors are welcome to fund and take the opportunity to become part of 10% share from our annual profits'
    },  
    {
        id: '4',
        description: 'Funding round is open for 90 days and will auto-close after deadline'
    },  
    {
        id: '5',
        description: 'Investors can claim funds back when funding target is not reached by deadline'
    },  
    {
        id: '6',
        description: 'Invested funds will automatically transferred to company reserves when funding is successful and claim will not be possible'
    },  
    {
        id: '7',
        description: 'Contact us if you have any questions or for any enquiries'
    },    
]
</script>
<template>    
    <v-dialog
        v-model="dialog"
        width="auto"
        transition="dialog-bottom-transition"
    >
        <template v-slot:activator="{ props }">
            <v-btn class="ml-2" rounded="pill" size="small" color="info" v-bind="props">
                Project Info
            </v-btn>
        </template>

        <v-card>
            <v-card-title>
                Project Information
            </v-card-title>
            <v-card-text>
                <v-list lines="one">
                    <v-list-item
                        v-for="item in projectInfo"
                        :key="item.id"
                        :title="item.description"                        
                    ></v-list-item>
                </v-list>
            </v-card-text>
            <v-card-actions>
                <v-btn color="primary" block @click="dialog = false">Close</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>    
  </template>