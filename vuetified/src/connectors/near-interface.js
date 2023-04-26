/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */

import { utils } from 'near-api-js'

export class Contract {

  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;
  }

  async getBeneficiary() {
    return await this.wallet.viewMethod({ contractId: this.contractId, method: "get_beneficiary" })
  }

  async latestDeposits() {
    const number_of_investors = await this.wallet.viewMethod({ contractId: this.contractId, method: "number_of_investors" })
    const min = number_of_investors > 10 ? number_of_investors - 9 : 0

    let deposits = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_deposits", args: { from_index: min.toString(), limit: number_of_investors } })

    deposits.forEach(elem => {
      elem.total_amount = utils.format.formatNearAmount(elem.total_amount);
    })

    return deposits
  }

  async getTargetAmount() {
    const target_amount = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_target" })
    
    return target_amount
  }

  async getFundingDeadline() {
    const deadline = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_deadline" })
    
    return deadline
  }

  async getDepositsTotal() {
    const depositsTotal = await this.wallet.viewMethod({ contractId: this.contractId, method: "deposits_total" })
    // console.log(depositsTotal / 10**24)
    return depositsTotal / 10**24
  }

  async getDonationFromTransaction(txhash) {
    let donation_amount = await this.wallet.getTransactionResult(txhash);
    return utils.format.formatNearAmount(donation_amount);
  }

  async deposit(amount) {
    let deposit = utils.format.parseNearAmount(amount.toString())
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "deposit", deposit })
    return response
  }

  async claim(amount) {
    // let deposit = utils.format.parseNearAmount(amount.toString())
    let response = ''
    try {
      response = await this.wallet.callMethod({ contractId: this.contractId, method: "claim", args: { claim_amount: amount } })      
    } catch (error) {
      response = error.message
    }
    
    return response
  }

  // async changeBeneficiary(newBeneficiary) {    
  //   let response = await this.wallet.callMethod({ contractId: this.contractId, method: "change_beneficiary", newBeneficiary })
  //   return response
  // }

}