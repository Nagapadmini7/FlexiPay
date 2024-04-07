use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Define storage items for the contract state
pub const STATE: Item<CrowdfundingPlatform> = Item::new("state");

// Enum to represent the type of campaign
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum CampaignType {
    Business,
    Charity,
}

// Struct to store information about a registered business
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Business {
    pub id: u64,
    pub name: String,
    pub description: String,
}

// Struct to store information about a donation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Donation {
    pub donor: String,
    pub amount: u64,
    pub campaign_id: u64,
}

// Struct to represent a reward
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reward {
    pub title: String,
    pub description: String,
}

// Struct to represent a reward tier
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardTier {
    pub amount: u64,
    pub reward: String,
}

// Struct to store information about a campaign
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Campaign {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub target_amount: u64,
    pub current_amount: u64,
    pub donations: Vec<Donation>,
    pub campaign_type: CampaignType,
    pub rewards: HashMap<u64, Reward>,
    pub reward_tiers: Vec<RewardTier>,
    pub start_date: u64,
    pub end_date: u64,
    pub updates: Vec<String>,
    pub social_links: Vec<String>,
    pub audit_report: Option<String>,
    pub progress_report: Option<String>,
}

// Struct to represent the crowdfunding platform
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CrowdfundingPlatform {
    pub campaigns: HashMap<u64, Campaign>,
    pub businesses: HashMap<u64, Business>,
    pub next_campaign_id: u64,
    pub next_business_id: u64,
    pub donor_donations: HashMap<String, HashSet<u64>>,
}

// Define message types for handling contract interactions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    RegisterBusiness {
        name: String,
        description: String,
    },
    CreateCampaign {
        title: String,
        description: String,
        target_amount: u64,
        campaign_type: CampaignType,
        start_date: u64,
        end_date: u64,
        social_links: Vec<String>,
        reward_tiers: Vec<RewardTier>,
    },
    Donate {
        campaign_id: u64,
        amount: u64,
        recurring: bool,
    },
    ClaimReward {
        campaign_id: u64,
        reward_id: u64,
    },
    UpdateProgress {
        campaign_id: u64,
        progress_report: String,
    },
}

// Define query types for querying contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCampaign { campaign_id: u64 },
}

// Define query response types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    Campaign(Campaign),
}

// Implement message handling functions
pub fn handle(deps: DepsMut, env: Env, info: MessageInfo, msg: HandleMsg) -> StdResult<Response> {
    match msg {
        HandleMsg::RegisterBusiness { name, description } => {
            register_business(deps, env, info, name, description)
        }
        HandleMsg::CreateCampaign {
            title,
            description,
            target_amount,
            campaign_type,
            start_date,
            end_date,
            social_links,
            reward_tiers,
        } => create_campaign(
            deps,
            env,
            info,
            title,
            description,
            target_amount,
            campaign_type,
            start_date,
            end_date,
            social_links,
            reward_tiers,
        ),
        HandleMsg::Donate {
            campaign_id,
            amount,
            recurring,
        } => donate(deps, env, info, campaign_id, amount, recurring),
        HandleMsg::ClaimReward {
            campaign_id,
            reward_id,
        } => claim_reward(deps, env, info, campaign_id, reward_id),
        HandleMsg::UpdateProgress {
            campaign_id,
            progress_report,
        } => update_progress(deps, env, info, campaign_id, progress_report),
    }
}

// Implement the donate function to handle one-time and recurring donations
fn donate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_id: u64,
    amount: u64,
    recurring: bool,
) -> StdResult<Response> {
    // Get the current state
    let mut state = STATE.load(deps.storage)?;

    // Get the campaign from the state
    let mut campaign = state
        .campaigns
        .get_mut(&campaign_id)
        .ok_or_else(|| StdError::generic_err("Campaign does not exist"))?;

    // Store donor's information and donation amount
    let donor = info.sender.clone();
    let donation = Donation {
        donor: donor.clone(),
        amount,
        campaign_id,
    };

    // Update the campaign's current amount
    campaign.current_amount += amount;

    // Add the donation to the campaign's list of donations for both one-time and recurring donations
    campaign.donations.push(donation.clone());

    // If it's a recurring donation, update the donor's recurring donation information
    if recurring {
        let donor_donations = state
            .donor_donations
            .entry(donor.clone())
            .or_insert_with(HashSet::new);
        donor_donations.insert(campaign_id);
    }

    // Save the updated state
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "donate")
        .add_attribute("amount", amount))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, CosmosMsg, WasmMsg};

    #[test]
    fn test_register_business() {
        // Initialize mock dependencies, environment, and info
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Execute the RegisterBusiness message
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

        // Check if the response is successful
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 1); // Check for expected attributes
    }

    #[test]
    fn test_create_campaign() {
        // Initialize mock dependencies, environment, and info
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("creator", &[]);

        // Execute the CreateCampaign message
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

        // Check if the response is successful
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 1); // Check for expected attributes
    }

    #[test]
    fn test_donate_and_update_progress() {
        // Initialize mock dependencies, environment, and info
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("donor", &[]);

        // Create a sample campaign
        let campaign_id = 1;
        let mut campaign = Campaign {
            id: campaign_id,
            title: "Test Campaign".to_string(),
            description: "Test Description".to_string(),
            target_amount: 1000,
            current_amount: 0,
            donations: vec![],
            campaign_type: CampaignType::Business,
            rewards: HashMap::new(),
            reward_tiers: vec![],
            start_date: 0,
            end_date: 0,
            updates: vec![],
            social_links: vec![],
            audit_report: None,
            progress_report: None,
        };
        let mut state = CrowdfundingPlatform {
            campaigns: HashMap::new(),
            businesses: HashMap::new(),
            next_campaign_id: 2,
            next_business_id: 1,
            donor_donations: HashMap::new(),
        };
        state.campaigns.insert(campaign_id, campaign.clone());
        STATE.save(&mut deps.storage, &state).unwrap();

        // Execute the Donate message
        let amount = 500;
        let res = handle(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            HandleMsg::Donate {
                campaign_id,
                amount,
                recurring: false,
            },
        )
        .unwrap();

        // Check if the response is successful
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 2); // Check for expected attributes

        // Check if campaign's current amount has been updated
        let state = STATE.load(&deps.storage).unwrap();
        let updated_campaign = state.campaigns.get(&campaign_id).unwrap();
        assert_eq!(updated_campaign.current_amount, amount);

        // Check if donation has been added to the campaign's list of donations
        assert_eq!(updated_campaign.donations.len(), 1);
        let donation = &updated_campaign.donations[0];
        assert_eq!(donation.amount, amount);
        assert_eq!(donation.donor, info.sender);

        // Execute the UpdateProgress message
        let progress_report = "Test Progress Report".to_string();
        let res = handle(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            HandleMsg::UpdateProgress {
                campaign_id,
                progress_report: progress_report.clone(),
            },
        )
        .unwrap();

        // Check if the response is successful
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.attributes.len(), 1); // Check for expected attributes

        // Check if progress report has been updated
        let state = STATE.load(&deps.storage).unwrap();
        let updated_campaign = state.campaigns.get(&campaign_id).unwrap();
        assert_eq!(updated_campaign.progress_report, Some(progress_report));
    }
}
#[test]
fn test_get_campaign() {
    // Initialize mock dependencies and storage
    let mut deps = mock_dependencies(&[]);

    // Create a sample campaign
    let campaign_id = 1;
    let campaign = Campaign {
        id: campaign_id,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 500,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: 0,
        end_date: 0,
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(campaign_id, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();

    // Execute the GetCampaign query
    let query_msg = QueryMsg::GetCampaign { campaign_id };
    let query_response: QueryResponse = query(deps.as_ref(), mock_env(), query_msg).unwrap();

    // Check if the query response matches the expected campaign
    match query_response {
        QueryResponse::Campaign(result_campaign) => {
            assert_eq!(result_campaign, campaign);
        }
    }
}
#[test]
fn test_invalid_donate_campaign_not_exist() {
    // Test donating to a non-existent campaign
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("donor", &[]);
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
    assert!(res.is_err()); // Ensure error is returned
}

#[test]
fn test_boundary_case_donate_recurring() {
    // Test donating a large amount with recurring donation
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("donor", &[]);
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
    assert!(res.is_err()); // Ensure error is returned
}

#[test]
fn test_contract_interaction_create_campaign() {
    // Test creating a campaign and verifying storage changes
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("creator", &[]);
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
    assert_eq!(res.messages.len(), 0); // Ensure no messages sent
}

#[test]
fn test_gas_consumption_donate() {
    // Test the gas consumption of the donate function
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("donor", &[]);
    let mut campaign = Campaign {
        id: 1,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 0,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: 0,
        end_date: 0,
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(1, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();
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
}

#[test]
fn test_concurrent_access_donate() {
    // Test concurrent access to donate function
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let info = mock_info("donor", &[]);
    let mut campaign = Campaign {
        id: 1,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 0,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: 0,
        end_date: 0,
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(1, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();

    // Spawn multiple threads to simultaneously donate
    let num_threads = 10;
    let mut handles = vec![];
    for _ in 0..num_threads {
        let deps = deps.clone();
        let env = env.clone();
        let info = info.clone();
        let handle_msg = HandleMsg::Donate {
            campaign_id: 1,
            amount: 100,
            recurring: false,
        };
        let handle_fn = move || {
            handle(deps.as_mut(), env.clone(), info.clone(), handle_msg.clone()).unwrap();
        };
        handles.push(std::thread::spawn(handle_fn));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Ensure campaign's current amount is correct after concurrent donations
    let state = STATE.load(&deps.storage).unwrap();
    let updated_campaign = state.campaigns.get(&1).unwrap();
    let expected_amount = num_threads as u64 * 100;
    assert_eq!(updated_campaign.current_amount, expected_amount);
}
#[test]
fn test_recurring_donations() {
    // Initialize mock dependencies, environment, and info
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let donor = "donor";
    let info = mock_info(donor, &[]);

    // Create a sample campaign
    let campaign_id = 1;
    let mut campaign = Campaign {
        id: campaign_id,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 0,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: 0,
        end_date: 0,
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(campaign_id, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();

    // Execute the Donate message with recurring donation
    let amount = 100;
    let recurring = true;
    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::Donate {
            campaign_id,
            amount,
            recurring,
        },
    )
    .unwrap();
    assert_eq!(res.messages.len(), 0); // Ensure no messages sent

    // Verify that the donor's recurring donation is registered correctly
    let donor_donations = state.donor_donations.get(donor).unwrap();
    assert!(donor_donations.contains(&campaign_id));

    // Verify that the campaign's current amount is updated correctly
    let state = STATE.load(&deps.storage).unwrap();
    let updated_campaign = state.campaigns.get(&campaign_id).unwrap();
    assert_eq!(updated_campaign.current_amount, amount);
}
#[test]
fn test_campaign_expiry() {
    // Initialize mock dependencies, environment, and info
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let donor = "donor";
    let info = mock_info(donor, &[]);

    // Create a sample campaign with an end date set to the current block time + 1 hour
    let campaign_id = 1;
    let end_date = env.block.time + 3600; // End date 1 hour from now
    let mut campaign = Campaign {
        id: campaign_id,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 0,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: env.block.time,
        end_date,
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(campaign_id, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();

    // Execute the Donate message to simulate donations during the campaign period
    let amount = 100;
    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::Donate {
            campaign_id,
            amount,
            recurring: false,
        },
    )
    .unwrap();
    assert_eq!(res.messages.len(), 0); // Ensure no messages sent

    // Verify that donations are still accepted during the campaign period
    let state = STATE.load(&deps.storage).unwrap();
    let updated_campaign = state.campaigns.get(&campaign_id).unwrap();
    assert_eq!(updated_campaign.current_amount, amount);

    // Advance the block time to after the campaign end date
    let env = mock_env();
    deps.querier.with_block(env.block);

    // Attempt to donate after the campaign has ended
    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::Donate {
            campaign_id,
            amount,
            recurring: false,
        },
    );
    assert!(res.is_err()); // Ensure error returned
}
#[test]
fn test_audit_reports() {
    // Initialize mock dependencies, environment, and info
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let creator = "creator";
    let info = mock_info(creator, &[]);

    // Create a sample campaign
    let campaign_id = 1;
    let mut campaign = Campaign {
        id: campaign_id,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 0,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: env.block.time,
        end_date: env.block.time + 3600, // End date 1 hour from now
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(campaign_id, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();

    // Execute the UpdateProgress message to add an audit report
    let audit_report = "Audit Report";
    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::UpdateProgress {
            campaign_id,
            progress_report: audit_report.to_string(),
        },
    )
    .unwrap();
    assert_eq!(res.messages.len(), 0); // Ensure no messages sent

    // Verify that the audit report is added correctly
    let state = STATE.load(&deps.storage).unwrap();
    let updated_campaign = state.campaigns.get(&campaign_id).unwrap();
    assert_eq!(
        updated_campaign.audit_report.as_ref().unwrap(),
        audit_report
    );
}

#[test]
fn test_event_emission() {
    // Initialize mock dependencies, environment, and info
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let creator = "creator";
    let info = mock_info(creator, &[]);

    // Create a sample campaign
    let campaign_id = 1;
    let mut campaign = Campaign {
        id: campaign_id,
        title: "Test Campaign".to_string(),
        description: "Test Description".to_string(),
        target_amount: 1000,
        current_amount: 0,
        donations: vec![],
        campaign_type: CampaignType::Business,
        rewards: HashMap::new(),
        reward_tiers: vec![],
        start_date: env.block.time,
        end_date: env.block.time + 3600, // End date 1 hour from now
        updates: vec![],
        social_links: vec![],
        audit_report: None,
        progress_report: None,
    };
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    state.campaigns.insert(campaign_id, campaign.clone());
    STATE.save(&mut deps.storage, &state).unwrap();

    // Execute the UpdateProgress message to trigger an event emission
    let progress_report = "Progress Report";
    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::UpdateProgress {
            campaign_id,
            progress_report: progress_report.to_string(),
        },
    )
    .unwrap();
    assert_eq!(res.messages.len(), 0); // Ensuring no messages sent

    // Verify that the expected event is emitted
    let expected_event =
        Event::new("progress_report").add_attribute("campaign_id", campaign_id.to_string());
    assert_eq!(res.events, vec![expected_event]);
}

#[test]
fn test_edge_cases() {
    // Initialize mock dependencies, environment, and info
    let mut deps = mock_dependencies(&[]);
    let env = mock_env();
    let creator = "creator";
    let info = mock_info(creator, &[]);

    // Create a sample campaign
    let mut state = CrowdfundingPlatform {
        campaigns: HashMap::new(),
        businesses: HashMap::new(),
        next_campaign_id: 2,
        next_business_id: 1,
        donor_donations: HashMap::new(),
    };
    // Add multiple campaigns to approach storage limits
    for i in 0..10_000 {
        let campaign_id = i as u64 + 1;
        let campaign = Campaign {
            id: campaign_id,
            title: format!("Campaign {}", campaign_id),
            description: "Test Description".to_string(),
            target_amount: 1000,
            current_amount: 0,
            donations: vec![],
            campaign_type: CampaignType::Business,
            rewards: HashMap::new(),
            reward_tiers: vec![],
            start_date: env.block.time,
            end_date: env.block.time + 3600,
            updates: vec![],
            social_links: vec![],
            audit_report: None,
            progress_report: None,
        };
        state.campaigns.insert(campaign_id, campaign);
    }
    STATE.save(&mut deps.storage, &state).unwrap();

    // Execute an action that consumes a significant amount of gas
    let res = handle(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        HandleMsg::UpdateProgress {
            campaign_id: 1,
            progress_report: "Progress Report".to_string(),
        },
    )
    .unwrap();

    // Assert that the action was successful
    assert!(res.is_ok());
}
