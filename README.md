# Flexi-Pay Crowdfunding: Empowering Users to Raise Funds


This repository contains a smart contract implementation for a crowdfunding platform using the CosmWasm framework. The contract enables users to register businesses, create campaigns, accept donations, claim rewards, and update campaign progress.

## Overview

The project is structured as follows:

- **`src/contract.rs`**:  Primary Rust file housing the core logic of the smart contract, including message handling functions, storage structures, query types, and testing suites.
- **Dependencies**: Utilizes external crates such as `cosmwasm_std`, `cw_storage_plus`, `schemars`, `serde`, and `serde_derive`.
- **Unit Tests**: Comprehensive tests cover various functionalities, edge cases, gas consumption, concurrency, etc.

## How it Works

1. **Storage Structs**: Define structs representing the state of the crowdfunding platform, including campaigns, businesses, donations, rewards, and reward tiers. These are stored in the contract's storage.

2. **Message Handling Functions**: Implement message handling functions (`handle`) to process different types of messages sent to the contract. Actions include registering businesses, creating campaigns, processing donations, updating progress, and claiming rewards.

3. **Query Types and Responses**: Define query types (`QueryMsg`) for querying the contract state and corresponding response types (`QueryResponse`) to provide requested data.

4. **Tests**: Comprehensive unit tests validate the contract's functionality, covering scenarios like registering businesses, creating campaigns, donating, updating progress, querying campaigns, handling invalid inputs, testing gas consumption, concurrency, campaign expiry, audit reports, and edge cases.

## Setup Instructions

To set up the project locally:

1. **Clone the Repository**: Clone the repository containing the project code to your local machine.

2. **Install Rust and Cargo**: Ensure Rust and Cargo (Rust's package manager) are installed on your machine from the [official Rust website](https://www.rust-lang.org/tools/install). Make sure the Cargo version is 1.17 or above.

3. **Install Dependencies**: Navigate to the project directory and run `cargo build` to install the required dependencies specified in `Cargo.toml`.

4. **Run Tests**: Execute `cargo test` to run the unit tests and verify the contract's functionality. Ensure all tests pass without errors.

5. **Demo**: Optionally, set up a demo environment where you can deploy the smart contract on a local blockchain or testnet, interact with it using simulated transactions, and showcase its functionalities.



# Crowdfunding Platform Documentation

This document provides an overview of the different function calls available in the codebase along with their descriptions and usage examples.

## Function Calls

### 1. `RegisterBusiness`
- **Description**: Registers a new business entity on the crowdfunding platform.
- **Parameters**:
  - `name`: Name of the business.
  - `description`: Description of the business.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::RegisterBusiness {
          name: "Test Business".to_string(),
          description: "Test Description".to_string(),
      },
  )
  .unwrap();
  ```

### 2. `CreateCampaign`
- **Description**: Creates a new campaign on the crowdfunding platform.
- **Parameters**:
  - `title`: Title of the campaign.
  - `description`: Description of the campaign.
  - `target_amount`: Target fundraising amount for the campaign.
  - `campaign_type`: Type of campaign (Business or Charity).
  - `start_date`: Start date of the campaign.
  - `end_date`: End date of the campaign.
  - `social_links`: Links to social media pages related to the campaign.
  - `reward_tiers`: Tiers of rewards offered for donations to the campaign.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::CreateCampaign {
          title: "Test Campaign".to_string(),
          description: "Test Description".to_string(),
          target_amount: 1000,
          campaign_type: CampaignType::Business,
          start_date: 0,
          end_date: 0,
          social_links: vec![],
          reward_tiers: vec![],
      },
  )
  .unwrap();
  ```

### 3. `Donate`
- **Description**: Allows users to donate funds to a specific campaign.
- **Parameters**:
  - `campaign_id`: ID of the campaign to donate to.
  - `amount`: Amount to donate.
  - `recurring`: Flag indicating whether the donation is recurring.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::Donate {
          campaign_id: 1,
          amount: 100,
          recurring: false,
      },
  )
  .unwrap();
  ```

### 4. `ClaimReward`
- **Description**: Allows users to claim a reward associated with a campaign.
- **Parameters**:
  - `campaign_id`: ID of the campaign from which to claim the reward.
  - `reward_id`: ID of the reward to claim.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::ClaimReward {
          campaign_id: 1,
          reward_id: 1,
      },
  )
  .unwrap();
  ```

### 5. `UpdateProgress`
- **Description**: Updates the progress report of a campaign.
- **Parameters**:
  - `campaign_id`: ID of the campaign to update.
  - `progress_report`: Progress report to be added.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::UpdateProgress {
          campaign_id: 1,
          progress_report: "Test Progress Report".to_string(),
      },
  )
  .unwrap();
  ```

### 6. `GetCampaign`
- **Description**: Retrieves information about a specific campaign.
- **Parameters**:
  - `campaign_id`: ID of the campaign to query.
- **Usage Example**:
  ```rust
  let query_msg = QueryMsg::GetCampaign { campaign_id: 1 };
  let query_response: QueryResponse = query(deps.as_ref(), mock_env(), query_msg).unwrap();
  ```

### 7. `Invalid Donate (Test)`
- **Description**: Tests donating to a non-existent campaign to trigger an error.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::Donate {
          campaign_id: 1, // Non-existent campaign ID
          amount: 100,
          recurring: false,
      },
  );
  ```

### 8. `Edge Case Donate (Test)`
- **Description**: Tests donating a large amount to a campaign to verify edge cases.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::Donate {
          campaign_id: 1,
          amount: u64::MAX, // Maximum donation amount
          recurring: true,
      },
  );
  ```

### 9. `Contract Interaction CreateCampaign (Test)`
- **Description**: Tests creating a campaign and verifying storage changes.
- **Usage Example**:
  ```rust
  let res = handle(
      deps.as_mut(),
      env.clone(),
      info.clone(),
      HandleMsg::CreateCampaign {
          title: "Test Campaign".to_string(),
          description: "Test Description".to_string(),
          target_amount: 1000,
          campaign_type: CampaignType::Business,
          start_date: 0,
          end_date: 0,
          social_links: vec![],
          reward_tiers: vec![],
      },
  )
  .unwrap();
  ```

These function calls cover various scenarios and edge cases, ensuring the robustness and correctness of the crowdfunding platform smart contract.

## Conclusion

This project provides a robust implementation of a crowdfunding platform smart contract using the CosmWasm framework. It adheres to best practices for smart contract development, including comprehensive testing and documentation. By setting it up locally and following the provided steps, you can effectively present and demonstrate its features and capabilities to others.


## Presentation Link: 
- <span style="color:#4CAF50">[Presentation]()</span>


## Contributors

- <span style="color:#4CAF50">[Khusbu](https://github.com/khushbusandhu)</span>
- <span style="color:#4CAF50">[Bucheti Naga Padmini](https://github.com/Nagapadmini7)</span>
- <span style="color:#4CAF50">[Akshansh Kaushal](https://github.com/Akshanshkaushal)</span>
- <span style="color:#4CAF50">[Sarthak Mishra](https://github.com/Sarthak-Mishra-5)</span>


## License

This project is licensed under the <span style="color:#4CAF50">[MIT License](LICENSE)</span>.

---
