#[path = ""]
pub mod google {
    #[path = "google.r#type.rs"]
    pub mod r#type;
}

#[path = ""]
pub mod systemathics {

    #[path = ""]
    pub mod apis {

        #[path = ""]
        pub mod r#type {

            #[path = ""]
            pub mod shared {

                #[path = "systemathics.apis.r#type.shared.v1.rs"]
                pub mod v1;
            }
        }

        #[path = ""]
        pub mod services {

            #[path = ""]
            pub mod corporate_actions {

                #[path = "systemathics.apis.services.corporate_actions.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod daily {

                #[path = "systemathics.apis.services.daily.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod daily_analytics {

                #[path = "systemathics.apis.services.daily_analytics.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod indices {

                #[path = "systemathics.apis.services.indices.v1.rs"]
                pub mod v1;
            }

            #[cfg(feature = "intraday")]
            #[path = ""]
            pub mod intraday {

                #[path = "systemathics.apis.services.intraday.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod intraday_analytics {

                #[path = "systemathics.apis.services.intraday_analytics.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod static_data {

                #[path = "systemathics.apis.services.static_data.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod sustainability {

                #[path = "systemathics.apis.services.sustainability.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod tick {

                #[path = "systemathics.apis.services.tick.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod tick_analytics {

                #[path = "systemathics.apis.services.tick_analytics.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod tick_conditions {

                #[path = "systemathics.apis.services.tick_conditions.v1.rs"]
                pub mod v1;
            }

            #[path = ""]
            pub mod topology {

                #[path = "systemathics.apis.services.topology.v1.rs"]
                pub mod v1;
            }
        }
    }
}
