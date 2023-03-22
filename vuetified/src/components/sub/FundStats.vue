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
            <th class="text-left">
            Total Fund Deposited Ⓝ
            </th>
        </tr>
        </thead>
        <tbody>
        <tr
            v-for="item in props.deposits"
            :key="item.account_id"
        >
            <td>{{ item.account_id }}</td>
            <td>{{ item.total_amount }}</td>
        </tr>
        </tbody>
    </v-table>
</template>