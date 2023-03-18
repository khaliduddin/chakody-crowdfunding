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

  async latestDonations() {
    const number_of_donors = await this.wallet.viewMethod({ contractId: this.contractId, method: "number_of_investors" })
    const min = number_of_donors > 10 ? number_of_donors - 9 : 0

    let donations = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_deposits", args: { from_index: min.toString(), limit: number_of_donors } })

    donations.forEach(elem => {
      elem.total_amount = utils.format.formatNearAmount(elem.total_amount);
    })

    return donations
  }

  async getTargetAmount() {
    const target_amount = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_target" })
    
    return target_amount
  }

  async getFundingDeadline() {
    const deadline = await this.wallet.viewMethod({ contractId: this.contractId, method: "get_deadline" })
    
    return deadline
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

  async changeBeneficiary(newBeneficiary) {    
    let response = await this.wallet.callMethod({ contractId: this.contractId, method: "change_beneficiary", newBeneficiary })
    return response
  }

}