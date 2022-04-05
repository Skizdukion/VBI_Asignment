#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod asignment5 {

    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{traits::SpreadAllocate, Mapping};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        QuestionAlreadyExists,
        QuestionNotExists,
        AnswerNotExists,
        QuestionIsClose,
        QuestionOrAnswerNotExist,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct CreateQuestion {
        #[ink(topic)]
        question: String,
        from: AccountId,
    }

    #[ink(event)]
    pub struct AnswerQuestion {
        #[ink(topic)]
        answer: String,
        question: String,
        from: AccountId,
    }

    #[ink(event)]
    pub struct UpvoteQOrA {
        #[ink(topic)]
        q_or_a: String,
        from: AccountId,
    }

    #[ink(event)]
    pub struct DownvoteQOrA {
        #[ink(topic)]
        q_or_a: String,
        from: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Asignment5 {
        question_to_owner: Mapping<String, AccountId>,
        answer_to_owner: Mapping<String, AccountId>,
        // answer_to_question: Mapping<String, String>,
        q_and_a_upvote: Mapping<String, Vec<AccountId>>,
        q_and_a_downvote: Mapping<String, Vec<AccountId>>,
        default_address: AccountId,
    }

    impl Asignment5 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.default_address = Default::default();
            })
        }

        #[ink(message)]
        pub fn create_question(&mut self, question: String) -> Result<()> {
            let caller = self.env().caller();
            if self.question_to_owner.get(&question).is_some() {
                return Err(Error::QuestionAlreadyExists);
            }
            self.question_to_owner.insert(&question, &caller);
            self.q_and_a_upvote
                .insert(&question, &Vec::<AccountId>::new());
            self.q_and_a_downvote
                .insert(&question, &Vec::<AccountId>::new());
            self.env().emit_event(CreateQuestion {
                question,
                from: caller,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn answer_to_question(&mut self, answer: String, question: String) -> Result<()> {
            let caller = self.env().caller();
            if self.question_to_owner.get(&question).is_none() {
                return Err(Error::QuestionNotExists);
            }
            self.answer_to_owner.insert(&answer, &caller);
            self.q_and_a_upvote
                .insert(&answer, &Vec::<AccountId>::new());
            self.q_and_a_downvote
                .insert(&answer, &Vec::<AccountId>::new());
            // self.answer_to_question.insert(&answer, &question);
            self.env().emit_event(AnswerQuestion {
                answer,
                question,
                from: caller,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn upvote_question(&mut self, question: String) -> Result<()> {
            let caller = self.env().caller();
            if self.question_to_owner.get(&question).is_none() {
                return Err(Error::QuestionNotExists);
            } else {
                self.upvoting(question, caller);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn downvote_question(&mut self, question: String) -> Result<()> {
            let caller = self.env().caller();
            if self.question_to_owner.get(&question).is_none() {
                return Err(Error::QuestionNotExists);
            } else {
                self.downvoting(question, caller);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn upvote_answer(&mut self, answer: String) -> Result<()> {
            let caller = self.env().caller();
            if self.answer_to_owner.get(&answer).is_none() {
                return Err(Error::AnswerNotExists);
            } else {
                self.upvoting(answer, caller);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn downvote_anwser(&mut self, answer: String) -> Result<()> {
            let caller = self.env().caller();
            if self.answer_to_owner.get(&answer).is_none() {
                return Err(Error::AnswerNotExists);
            } else {
                self.downvoting(answer, caller);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn get_voting_point(&self, q_or_a: String) -> Option<i128> {
            let upvoting_list = self.q_and_a_upvote.get(&q_or_a);
            let downvoting_list = self.q_and_a_downvote.get(&q_or_a);
            if downvoting_list.is_none() || upvoting_list.is_none() {
                return None;
            }

            Some(upvoting_list.unwrap().len() as i128 - downvoting_list.unwrap().len() as i128)
        }

        fn upvoting(&mut self, q_or_a: String, caller: AccountId) {
            let mut upvoting_list = self.q_and_a_upvote.get(&q_or_a).unwrap();
            let mut downvoting_list = self.q_and_a_downvote.get(&q_or_a).unwrap();
            // upvoting_list.push(caller);
            // self.q_and_a_upvote.insert(&q_or_a, &upvoting_list);
            if let Some(index) = downvoting_list.iter().position(|&r| r == caller) {
                downvoting_list.remove(index);
                self.q_and_a_downvote.insert(&q_or_a, &downvoting_list);
            }
            if let None = upvoting_list.iter().position(|&r| r == caller) {
                upvoting_list.push(caller);
                self.q_and_a_upvote.insert(&q_or_a, &upvoting_list);
            }

            self.env().emit_event(UpvoteQOrA {
                q_or_a,
                from: caller,
            });
        }

        fn downvoting(&mut self, q_or_a: String, caller: AccountId) {
            let mut upvoting_list = self.q_and_a_upvote.get(&q_or_a).unwrap();
            let mut downvoting_list = self.q_and_a_downvote.get(&q_or_a).unwrap();
            // downvoting_list.push(caller);
            // self.q_and_a_downvote.insert(&q_or_a, &upvoting_list);
            if let Some(index) = upvoting_list.iter().position(|&r| r == caller) {
                upvoting_list.remove(index);
                self.q_and_a_upvote.insert(&q_or_a, &upvoting_list);
            }
            if let None = downvoting_list.iter().position(|&r| r == caller) {
                downvoting_list.push(caller);
                self.q_and_a_downvote.insert(&q_or_a, &downvoting_list);
            }
            self.env().emit_event(DownvoteQOrA {
                q_or_a,
                from: caller,
            });
        }
    }
}
