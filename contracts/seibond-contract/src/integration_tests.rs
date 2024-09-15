#[cfg(test)]
mod tests {
    use crate::msg::{InstantiateMsg, ExecuteMsg};
    use crate::helpers::CwTemplateContract;
    use crate::state::Bond;
    use cosmwasm_std::testing::MockApi;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const PLATFORM_TREASURY: &str = "platform_treasury";
    const NATIVE_DENOM: &str = "denom";
    const SEIX_TOKEN: &str = "seix_token"; // Token address for SeiX

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &MockApi::default().addr_make(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let user = app.api().addr_make(USER);
        assert_eq!(
            app.wrap().query_balance(user, NATIVE_DENOM).unwrap().amount,
            Uint128::new(1)
        );

        let msg = InstantiateMsg {
            seix_contract: SEIX_TOKEN.to_string(), // Use SeiX token contract address
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod bond_tests {
        use super::*;

        #[test]
        fn mint_bond() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Test minting a bond
            let msg = ExecuteMsg::MintBond {
                face_value: Uint128::new(1000),
                interest_rate: 5,
                maturity: 3600,
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // You could query the bond state and verify the bond was correctly minted
        }

        #[test]
        fn transfer_bond() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Mint a bond first
            let mint_msg = ExecuteMsg::MintBond {
                face_value: Uint128::new(1000),
                interest_rate: 5,
                maturity: 3600,
            };
            let cosmos_msg = cw_template_contract.call(mint_msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // Test transferring the bond
            let transfer_msg = ExecuteMsg::TransferBond {
                bond_id: "1".to_string(),
                new_holder: Addr::unchecked(PLATFORM_TREASURY),
            };
            let cosmos_msg = cw_template_contract.call(transfer_msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }

        #[test]
        fn redeem_bond() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // Mint a bond first
            let mint_msg = ExecuteMsg::MintBond {
                face_value: Uint128::new(1000),
                interest_rate: 5,
                maturity: 3600,
            };
            let cosmos_msg = cw_template_contract.call(mint_msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            // Fast-forward time to simulate bond maturity
            app.update_block(|block| {
                block.time = block.time.plus_seconds(3601); // Simulate 3600 seconds
            });

            // Test redeeming the bond
            let redeem_msg = ExecuteMsg::RedeemBond {
                bond_id: "1".to_string(),
            };
            let cosmos_msg = cw_template_contract.call(redeem_msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
