use rust_macios::{
    contacts::{
        CNContactFetchRequest, CNContactFormatter, CNContactFormatterStyle,
        CNContactPhoneNumbersKey, CNContactStore, CNEntityType, ICNContactStore,
    },
    foundation::{INSEnumerator, NSError},
    nsarray,
    objective_c_runtime::traits::PNSObject,
};

fn main() {
    // Create a new `CNContactStore` to fetch the contacts from the user's
    // contact database.
    let store = CNContactStore::m_new();

    // Creates a clone of the `CNContactStore` so we can use it after we
    // request access to the user's contact database
    let captured_store = store.clone();

    // Requests access to the users contact database
    store.request_access_for_entity_type_completion_handler(
        CNEntityType::Contacts,
        move |granted: bool, _error: *mut NSError| {
            if granted {
                // Create an `NSArray` filled keys you'd like to fetch and use
                // in your program
                //
                // see more: <https://developer.apple.com/documentation/contacts/contact_keys?language=objc>
                let keys_to_fetch = nsarray![
                    CNContactFormatter::descriptor_for_required_keys_for_style(
                        CNContactFormatterStyle::FullName
                    ),
                    // Create an unsafe block for ay additional keys
                    unsafe {
                        // For each key, call the `m_self` function because the array
                        // needs to be of type `NSArray<*mut Object>` aka `NSArray<id>`
                        CNContactPhoneNumbersKey.m_self()
                    }
                ];

                // Creates the fetch request we would be using to partial
                // fetch data from the database
                let request =
                    CNContactFetchRequest::m_alloc().init_with_keys_to_fetch(keys_to_fetch);

                // We use this method becasue it's easiar to get an enumerator with it
                let fetch_result = captured_store
                    .m_enumerator_for_contact_fetch_request(request)
                    .unwrap();

                for contact in &fetch_result.value().m_all_objects() {
                    println!("{} {}", contact.given_name(), contact.family_name());

                    for phone_number in &contact.phone_numbers() {
                        println!("{}", phone_number.value().string_value())
                    }
                }
            }
        },
    );
}
