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
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> StdResult<Response> {
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
        } => {
            create_campaign(
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
            )
        }
        HandleMsg::Donate {
            campaign_id,
            amount,
            recurring,
        } => donate(deps, env, info, campaign_id, amount, recurring),
        HandleMsg::ClaimReward { campaign_id, reward_id } => {
            claim_reward(deps, env, info, campaign_id, reward_id)
        }
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

    // Handle one-time donation
    if !recurring {
        // Store donor's information and donation amount for one-time donations
        let donor = info.sender.clone();
        let donation = Donation {
            donor: donor.clone(),
            amount,
            campaign_id,
        };

        // Update the campaign's current amount
        campaign.current_amount += amount;

        // Add the donation to the campaign's list of donations
        campaign.donations.push(donation);
    } else {
        // Handle recurring donation
        // Store donor's information and donation amount for recurring donations
        let donor = info.sender.clone();
        let donation = Donation {
            donor: donor.clone(),
            amount,
            campaign_id,
        };

        // Update the donor's recurring donation information
        let donor_donations = state
            .donor_donations
            .entry(donor.clone())
            .or_insert_with(HashSet::new);
        donor_donations.insert(campaign_id);

        // Update the campaign's current amount
        campaign.current_amount += amount;
    }