# sieble_file_splitter
A program written in Rust to split sieble CSV file into multiple JSON files

### data model
lendingSubmission.submissionId

lendingSubmission.dateCreated

lendingSubmission.Version

lendingSubmission.displayStatus

lendingSubmission.submissionReason

lendingSubmission.ApprovalDate

lendingSubmission.groupId

lendingSubmission.name

lendingSubmission.buid

lendingSubmission.primaryPositionId

lendingSubmission.primaryPositionBuid

lendingSubmission.empFirstName

lendingSubmission.empLastName

lendingSubmission.borrowers.0.borrowerId

lendingSubmission.borrowers.0.name

lendingSubmission.borrowers.0.borrowerNumber

lendingSubmission.borrowers.0.primaryOrgRowId

lendingSubmission.borrowers.0.primaryOrgCustId

lendingSubmission.borrowers.0.orgCrs

lendingSubmission.borrowers.0.orgIndustryRowId

lendingSubmission.borrowers.0.orgIndustry1

lendingSubmission.borrowers.0.orgIndustry2

lendingSubmission.borrowers.0.orgIndustry3

lendingSubmission.borrowers.0.primaryOrgIndustryClass

lendingSubmission.borrowers.0.primaryContactRowId

lendingSubmission.borrowers.0.primaryContactCustId

lendingSubmission.borrowers.0.IndCRS

lendingSubmission.borrowers.0.contactOccuRowId

lendingSubmission.borrowers.0.contactOccu

lendingSubmission.borrowers.0.contactOccu1

lendingSubmission.borrowers.0.contactOccu2

lendingSubmission.borrowers.0.primaryContactOccupation

lendingSubmission.borrowers.0.contactOccuCode

lendingSubmission.borrowers.0.facilitiesAndProducts.0.productId

lendingSubmission.borrowers.0.facilitiesAndProducts.0.name

lendingSubmission.borrowers.0.facilitiesAndProducts.0.existingLimit

lendingSubmission.borrowers.0.facilitiesAndProducts.0.lendingCategory

lendingSubmission.borrowers.0.facilitiesAndProducts.0.expiry

lendingSubmission.borrowers.0.facilitiesAndProducts.0.soughtAmount

lendingSubmission.borrowers.0.facilitiesAndProducts.0.commCode

lendingSubmission.borrowers.0.facilitiesAndProducts.0.accountId

lendingSubmission.borrowers.0.facilitiesAndProducts.0.accountNumber

lendingSubmission.borrowers.0.securitiesAndCharges.0.chargeId

lendingSubmission.borrowers.0.securitiesAndCharges.0.chargeName

lendingSubmission.borrowers.0.securitiesAndCharges.0.chargeDescription

lendingSubmission.borrowers.0.securitiesAndCharges.0.amountOfGuarantee

lendingSubmission.borrowers.0.securitiesAndCharges.0.unlimitedFlg

lendingSubmission.borrowers.0.securitiesAndCharges.0.recordtype

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityId

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityType

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityValuationDate

lendingSubmission.borrowers.0.securitiesAndCharges.0.marketValue

lendingSubmission.borrowers.0.securitiesAndCharges.0.shading

lendingSubmission.borrowers.0.securitiesAndCharges.0.rmdTotBv

lendingSubmission.borrowers.0.securitiesAndCharges.0.primaryContactId

lendingSubmission.borrowers.0.securitiesAndCharges.0.primaryOrgId

lendingSubmission.borrowers.0.securitiesAndCharges.0.recordType

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHolders.0.contactId

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHolders.0.contactNum

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHolders.0.contactLastName

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHolders.0.contactFirstName

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHoldersOrg.0.orgId

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHoldersOrg.0.orgNum

lendingSubmission.borrowers.0.securitiesAndCharges.0.securityHoldersOrg.0.orgName

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.supportedChargeId

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.chargeName

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.chargeDescription

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.chargeRecordType

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.supportedSecurityId

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityType

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecValuationDate

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecMarketValue

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecShading

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecBankValue

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecPrimaryConId

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecPrimaryOrgId

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.suppSecRecType

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHolders.0.suppSecContactId

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHolders.0.suppSecContactNumber

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHolders.0.suppSecContactLastName

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHolders.0.suppSecContactFirstName

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHoldersOrg.0.suppSecOrgId

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHoldersOrg.0.suppSecOrgNum

lendingSubmission.borrowers.0.securitiesAndCharges.0.guarantees.0.securityHoldersOrg.0.suppSecOrgName
