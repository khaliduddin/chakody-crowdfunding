
const API_NEAR_USD = "https://api.coingecko.com/api/v3/simple/price?ids=near&vs_currencies=usd"

export const findNear2UsdPrice = async () => {        
    // depositMessage.value = '' // reset message
    let data = await fetch(API_NEAR_USD).then(response => response.json())
    const near2usd = data['near']['usd']
    // const amount_in_near = amount / near2usd
    // const rounded_two_decimals = Math.round(amount_in_near * 100) / 100
    // usdAmount.value = amount
    // usd2near.value = rounded_two_decimals
    return near2usd
}

export const roundToTwoDecimals = (num) => {
    return Math.round(num * 100) / 100
}