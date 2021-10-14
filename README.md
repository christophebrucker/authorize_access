
**Executive Summary**

This paper aims at explaining the specifications of the diabmate platform exchange protocol. Healthcare data are rare and expensive. [Diabmate](www.diabmate.com) is giving patients a tool to better manage diabetes type 1/type 2 and  share information helping physicians, dieticians, nurses and researchers to do their jobs.
The medical staff directly involved in the treatment have access to the patient’s Electronic Health Record (EHR).
Researchers can browse the anonymised information they would like to get and buy data from a market place.
Family and friends involved in the patient disease treatment can have access to information in order to better understand what they can do to help the patient.

**1.Diabmate Platform**

Diabetes management involve many persons from medical staff to patient’s relatives. Diabmate is a platform allowing the different persons retrieve the data they need.

Thanks to the platform, patients can write four data types :

1.glucose readings (time/value)
2.insuline intakes (time/value)
3.food (nutrients/time)
4.activities ( category/time ).

Glucose readings and insuline intakes may be obtained with sensors.

The endocrinologists, the dieticians, the relatives and the medical researchers can have access to selected information if the patient is authorising them.
The endocrinologists can write/modify prescriptions and send them to their patients.
The dieticians can write/modify meal plans and send them to their patients.

**2.Blockchain Technology**

Blockchain is a peer-to-peer (P2P) distributed ledger technology for transactional applications that establishes transparency and trust. Blockchain is the underlying fabric for Bitcoin and is a design pattern consisting of three main components: a distributed network, a shared ledger and digital transactions.

Distributed Network Blockchain is a decentralized P2P architecture with nodes consisting of network participants. Each member in the network stores an identical copy of the blockchain and contributes to the collective process of validating and certifying digital transactions for the network.

Shared Ledger - Members in the distributed network record digital transactions into a shared ledger. To add transactions, members in the network run algorithms to evaluate and verify the proposed transaction. If a majority of the members in the network agree that the transaction is valid, the new transaction is added to the shared ledger. Changes to the shared ledger are reflected in all copies of the blockchain in minutes or, in some cases, seconds. After a transaction is added it is immutable and cannot be changed or removed. Since all members in the network have a complete copy of the blockchain no single member has the power to tamper or alter data.

Digital Transaction - Any type of information or digital asset can be stored in a blockchain, and the network implementing the blockchain defines the type of information contained in the transaction. Information is encrypted and digitally signed to guarantee authenticity and accuracy. Transactions are structured into blocks and each block contains a cryptographic hash to the prior block in the blockchain. Blocks are added in a linear, chronological order.

**3.Proof of concept**

Our proposal involves the use of a blockchain as an access-control manager to health records that are stored off blockchain.We use the Substrate framework with the Ink! Embedded domain specific language to write the smart contracts between the different nodes of the blockchain.

The first element we want to build is a network of nodes. We assume that one node is representing a patient. We use the Substrate’s node-template including pallet-contracts so that we can have to capability to include smart contracts.

This patient has the possibility to authorize and revoke access to his data. To simplify the case, we write the smart contract has a simple boolean value being false by default. The smart contract written with Ink! gives the capability to change this value to true. The code for this contract is available at https://github.com/christophebrucker/authorize_access.

**4.Next Steps**

a.Evaluate pros/cons for the blockchain to be private / permissioned.

b.Define the property of the Healthcare data lake

c.Store on the blockchain the access granted by one patient as a tuple (Rust data type) : (patient_id, data_address, timestamps)



**Bibliography**

1.Bitcoin: A Peer-to-Peer Electronic Cash System - Satoshi Nakamoto - 2008

2.Ethereum White Paper - Updated version June 28th 2021

3.Polkadot : Vision for a Heterogeneous multi-chain-framework - draft 1- Dr. Gavin Wood

4.Attribute-based Multi-Signature and Encryption for EHR Management : a blockchain-based solution. Hao Guo - Wanxin Li - Ehsan Meamari - Chien-Chung Shen - Mark Nejad 1999
