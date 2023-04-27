
const API_NEAR_USD = "https://api.coingecko.com/api/v3/simple/price?ids=near&vs_currencies=usd"
const NANO_SECS_MULTIPLIER = 1000000

export const findNear2UsdPrice = async () => {   
    let data = await fetch(API_NEAR_USD).then(response => response.json())
    const near2usd = data['near']['usd']
    
    // const near2usd = await fetchPriceData()
    return near2usd
}

// export const findUsd2NearAmount = async (amount) => {
//     const near2usd = await fetchPriceData()
//     const amount_in_near = amount / near2usd
//     const rounded_two_decimals = Math.round(amount_in_near * 100) / 100
//     usdAmount.value = amount
//     usd2near.value = rounded_two_decimals
// }

// const fetchPriceData = async () => {
//     let data = await fetch(API_NEAR_USD).then(response => response.json())
//     return data['near']['usd']
// }

export const roundToTwoDecimals = (num) => {
    return Math.round(num * 100) / 100
}

export const convertToNanoDate = (daysToAdd) => {
    // let nanoDate = new Date(nanoDateFromContract / NANO_SECS_MULTIPLIER)
    let nanoDate = new Date()
    let timeInMillis = nanoDate.getTime()
    let timeInMillisWithAddedDays = 1000 * 60 * 60 * 24 * daysToAdd
    // console.log('date testing in millisecs 1', timeInMillis)
    // console.log('date testing in millisecs 2', timeInMillisWithAddedDays)
    let newTimeinMillis = timeInMillis + timeInMillisWithAddedDays
    // console.log('new millis ', newTimeinMillis)

    let newDate = new Date(newTimeinMillis)
    // console.log('new date ', newDate.toLocaleDateString("en-US"))

    // console.log('date testing ', nanoDate.toLocaleDateString("en-US"))
    // console.log('date part ', nanoDate.getDate())
    // console.log('days to add ', daysToAdd)
    // console.log('simple date + 90 days -> ', nanoDate.setDate(nanoDate.getDate() + daysToAdd))
    // console.log('date after update 1 ', nanoDate.toLocaleDateString("en-US"))
    // nanoDate = new Date(nanoDate.getTime() + (86400 * daysToAdd))
    // console.log('date after update 2 ', nanoDate.toLocaleDateString("en-US"))
    // closingDate.value = closingDate.value.toLocaleDateString("en-US")  
    return newDate * NANO_SECS_MULTIPLIER
}