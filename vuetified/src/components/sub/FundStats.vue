<script setup>
const props = defineProps(['deposits'])
</script>
<template>
    <v-table>
        <thead>
        <tr>
            <th class="text-left">
            Investor
            </th>
            <th class="text-center">
            Fund Deposit Ⓝ
            </th>
        </tr>
        </thead>
        <tbody>
        <tr
            v-for="item in props.deposits"
            :key="item.account_id"
        >
            <td class="text-left">{{ item.account_id }}</td>
            <td>{{ item.total_amount }}</td>
        </tr>
        </tbody>
    </v-table>
</template>