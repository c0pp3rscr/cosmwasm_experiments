#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::testing::MockApi;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        ).with_reply(crate::contract::reply);
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

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

        let msg: InstantiateMsg = InstantiateMsg { count: 1i32, contract_id : String::from("SDF") , a_or_b: 0, contract_to_call: String::new(), submsgid: 5};
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

    fn instantiate_modular() -> (App, CwTemplateContract, CwTemplateContract, CwTemplateContract) {

        let mut app: App = mock_app();
        let cw_template_id: u64 = app.store_code(contract_template());

        // contract A is supposed to call contract B
        let msg_a = InstantiateMsg { count: 1i32, contract_id: String::from("contract-A"), contract_to_call: String::new(), 
            a_or_b: 0, // it's meant to call function A of contract B ,
            submsgid: 1
                 };
        let contract_a_address = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg_a,
                &[],
                "test",
                None,
            )
            .unwrap();

        let msg_b = InstantiateMsg { count: 1i32, contract_id: String::from("contract-B"), contract_to_call: String::new(), 
        a_or_b: 0, submsgid: 2 }; // has to call function A of contract C
        let contract_b_address = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg_b,
                &[],
                "test",
                None,
            )
            .unwrap();

        let msg_c = InstantiateMsg { count: 1i32, contract_id: String::from("contract-C"), contract_to_call: String::new(), 
        a_or_b: 134,submsgid:3 }; // calls function B of contract B
        let contract_c_address = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg_c,
                &[],
                "test",
                None,
            )
            .unwrap();

        let contract_a = CwTemplateContract(contract_a_address);
        let contract_b = CwTemplateContract(contract_b_address);
        let contract_c = CwTemplateContract(contract_c_address);

        // app.change_contract_to_call();

        (app, contract_a, contract_b, contract_c)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Increment {};
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }

    mod multicall {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn multicall() {
            let (mut app, contract_a, contract_b, contract_c) = instantiate_modular();
            
            let msg_a_to_b = ExecuteMsg::Change_contract_to_call { new_contract_to_call: String::from(contract_b.addr()) };
            let msg_b_to_c = ExecuteMsg::Change_contract_to_call { new_contract_to_call: String::from(contract_c.addr()) };
            let msg_c_to_b = ExecuteMsg::Change_contract_to_call { new_contract_to_call: String::from(contract_b.addr()) };

            // let msg = ExecuteMsg::Increment {};
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();


            let mut cosmos_msg = contract_a.call(msg_a_to_b).unwrap();
            let res = app.execute(Addr::unchecked(USER), cosmos_msg.clone());
            assert!(res.is_ok(), "Execution failed: {:?}", res.unwrap_err());

            // print!("result is {:?} ", res);

            cosmos_msg = contract_b.call(msg_b_to_c).unwrap();
            let mut res = app.execute(Addr::unchecked(USER), cosmos_msg.clone());
            assert!(res.is_ok(), "Execution failed: {:?}", res.unwrap_err());

            cosmos_msg = contract_c.call(msg_c_to_b).unwrap();
            res = app.execute(Addr::unchecked(USER), cosmos_msg.clone());
            assert!(res.is_ok(), "Execution failed: {:?}", res.unwrap_err());

            let msg_execute_functiona_contracta = ExecuteMsg::FunctionA;
            cosmos_msg = contract_a.call(msg_execute_functiona_contracta).unwrap();
            res = app.execute(Addr::unchecked(USER), cosmos_msg.clone());
            assert!(res.is_ok(), "Execution failed: {:?}", res.unwrap_err());
            

        }
    }
}
