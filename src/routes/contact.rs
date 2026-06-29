use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};
use leptos_lucide_rs::ChevronsUpDown;

use crate::components::{PageLayout, PageLoader, PageSection, PageText};
use crate::config;
use crate::i18n::{self, MessageKey};
use crate::support_contact::{
    SupportContactSubmitResult, SupportContactValues, submit_support_contact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContactMethod {
    Email,
    Nostr,
}

impl ContactMethod {
    fn value(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Nostr => "nostr",
        }
    }

    fn from_value(value: &str) -> Option<Self> {
        match value {
            "email" => Some(Self::Email),
            "nostr" => Some(Self::Nostr),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContactSubmitState {
    Idle,
    Sending,
    Accepted,
    Duplicate,
    Error,
}

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <PageLayout>
            <PageSection>
                <PageText label=MessageKey::HomepageContactHeading />
                <PageText label=MessageKey::HomepageContactBody />
                <ContactForm />
            </PageSection>
        </PageLayout>
    }
}

#[component]
fn ContactForm() -> impl IntoView {
    let i18n = mf2_i18n::leptos::use_i18n();
    let (name, set_name) = signal(String::new());
    let (contact_address, set_contact_address) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (contact_method, set_contact_method) = signal(ContactMethod::Email);
    let (submitted, set_submitted) = signal(false);
    let (submit_state, set_submit_state) = signal(ContactSubmitState::Idle);
    let locale = i18n.locale_signal();
    let name_i18n = i18n.clone();
    let name_label = move || i18n::text(&name_i18n, MessageKey::HomepageContactFormNameLabel);
    let method_i18n = i18n.clone();
    let method_label = move || {
        i18n::text(
            &method_i18n,
            MessageKey::HomepageContactFormReplyMethodLabel,
        )
    };
    let email_i18n = i18n.clone();
    let email_label =
        move || i18n::text(&email_i18n, MessageKey::HomepageContactFormReplyMethodEmail);
    let nostr_i18n = i18n.clone();
    let nostr_label =
        move || i18n::text(&nostr_i18n, MessageKey::HomepageContactFormReplyMethodNostr);
    let address_i18n = i18n.clone();
    let message_i18n = i18n.clone();
    let message_label =
        move || i18n::text(&message_i18n, MessageKey::HomepageContactFormMessageLabel);
    let submit_i18n = i18n.clone();
    let name_error_i18n = i18n.clone();
    let address_error_i18n = i18n.clone();
    let message_error_i18n = i18n.clone();
    let submit_status_i18n = i18n.clone();
    let submit_status = move || {
        let key = match submit_state.get() {
            ContactSubmitState::Idle | ContactSubmitState::Sending => return None,
            ContactSubmitState::Accepted => MessageKey::HomepageContactFormStatusAccepted,
            ContactSubmitState::Duplicate => MessageKey::HomepageContactFormStatusDuplicate,
            ContactSubmitState::Error => MessageKey::HomepageContactFormStatusError,
        };

        Some(i18n::text(&submit_status_i18n, key))
    };
    let contact_address_label = move || {
        let key = if contact_method.get() == ContactMethod::Email {
            MessageKey::HomepageContactFormEmailLabel
        } else {
            MessageKey::HomepageContactFormPublicKeyLabel
        };
        i18n::text(&address_i18n, key)
    };
    let contact_address_type = move || {
        if contact_method.get() == ContactMethod::Email {
            "email"
        } else {
            "text"
        }
    };
    let contact_address_autocomplete = move || {
        if contact_method.get() == ContactMethod::Email {
            "email"
        } else {
            "off"
        }
    };
    let name_error = Signal::derive(move || {
        required_error(
            &name.get(),
            submitted.get(),
            MessageKey::HomepageContactFormValidationNameRequired,
        )
    });
    let contact_address_error = Signal::derive(move || {
        let contact_address = contact_address.get();
        let submitted = submitted.get();
        let method = contact_method.get();

        if let Some(error) = required_error(
            &contact_address,
            submitted,
            match method {
                ContactMethod::Email => MessageKey::HomepageContactFormValidationEmailRequired,
                ContactMethod::Nostr => MessageKey::HomepageContactFormValidationPublicKeyRequired,
            },
        ) {
            return Some(error);
        }

        if submitted && method == ContactMethod::Email && !is_valid_email(&contact_address) {
            return Some(MessageKey::HomepageContactFormValidationEmailInvalid);
        }

        None
    });
    let message_error = Signal::derive(move || {
        required_error(
            &message.get(),
            submitted.get(),
            MessageKey::HomepageContactFormValidationMessageRequired,
        )
    });
    let form_has_errors = move || {
        name_error.get().is_some()
            || contact_address_error.get().is_some()
            || message_error.get().is_some()
    };
    let handle_method_change = move |event| {
        if let Some(method) = ContactMethod::from_value(&event_target_value(&event)) {
            set_contact_method.set(method);
            set_submit_state.set(ContactSubmitState::Idle);
        }
    };
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_submitted.set(true);
        set_submit_state.set(ContactSubmitState::Idle);

        if form_has_errors() {
            return;
        }

        set_submit_state.set(ContactSubmitState::Sending);
        let values = SupportContactValues {
            display_name: name.get_untracked(),
            outreach_method: contact_method.get_untracked().value().to_owned(),
            contact_address: contact_address.get_untracked(),
            message: message.get_untracked(),
        };
        let locale = locale.get_untracked();

        spawn_local(async move {
            let result =
                submit_support_contact(config::RADROOTS_SUPPORT_CONTACT_URL, values, locale).await;
            set_submit_state.set(match result {
                SupportContactSubmitResult::Accepted => ContactSubmitState::Accepted,
                SupportContactSubmitResult::Duplicate => ContactSubmitState::Duplicate,
                SupportContactSubmitResult::Rejected | SupportContactSubmitResult::Unavailable => {
                    ContactSubmitState::Error
                }
            });
        });
    };

    view! {
        <form class="page-contact-form" method="post" on:submit=on_submit novalidate>
            <label class="page-form-field" for="contact-name">
                <span class="page-form-label">{name_label}</span>
                <span
                    class="page-form-control"
                    class:page-form-control-error=move || name_error.get().is_some()
                >
                    <input
                        id="contact-name"
                        class="page-form-input"
                        name="display_name"
                        type="text"
                        autocomplete="name"
                        disabled=move || submit_state.get() == ContactSubmitState::Sending
                        prop:value=move || name.get()
                        on:input=move |event| {
                            set_name.set(event_target_value(&event));
                            set_submit_state.set(ContactSubmitState::Idle);
                        }
                    />
                </span>
                {move || {
                    name_error.get().map(|key| view! {
                        <span class="page-form-error">{i18n::text(&name_error_i18n, key)}</span>
                    })
                }}
            </label>
            <label class="page-form-field" for="contact-method">
                <span class="page-form-label">{method_label}</span>
                <span class="page-form-control page-form-control-select">
                    <select
                        id="contact-method"
                        class="page-form-input page-form-input-select"
                        name="outreach_method"
                        disabled=move || submit_state.get() == ContactSubmitState::Sending
                        prop:value=move || contact_method.get().value()
                        on:change=handle_method_change
                    >
                        <option value="email">{email_label}</option>
                        <option value="nostr">{nostr_label}</option>
                    </select>
                    <span class="page-form-select-icon">
                        <ChevronsUpDown />
                    </span>
                </span>
            </label>
            <label class="page-form-field" for="contact-address">
                <span class="page-form-label">{contact_address_label}</span>
                <span
                    class="page-form-control"
                    class:page-form-control-error=move || contact_address_error.get().is_some()
                >
                    <input
                        id="contact-address"
                        class="page-form-input"
                        name="contact_address"
                        type=contact_address_type
                        autocomplete=contact_address_autocomplete
                        disabled=move || submit_state.get() == ContactSubmitState::Sending
                        prop:value=move || contact_address.get()
                        on:input=move |event| {
                            set_contact_address.set(event_target_value(&event));
                            set_submit_state.set(ContactSubmitState::Idle);
                        }
                    />
                </span>
                {move || {
                    contact_address_error.get().map(|key| view! {
                        <span class="page-form-error">{i18n::text(&address_error_i18n, key)}</span>
                    })
                }}
            </label>
            <label class="page-form-field" for="contact-message">
                <span class="page-form-label">{message_label}</span>
                <span
                    class="page-form-control page-form-control-tall"
                    class:page-form-control-error=move || message_error.get().is_some()
                >
                    <textarea
                        id="contact-message"
                        class="page-form-input page-form-input-textarea"
                        name="message"
                        rows="4"
                        disabled=move || submit_state.get() == ContactSubmitState::Sending
                        prop:value=move || message.get()
                        on:input=move |event| {
                            set_message.set(event_target_value(&event));
                            set_submit_state.set(ContactSubmitState::Idle);
                        }
                    ></textarea>
                </span>
                {move || {
                    message_error.get().map(|key| view! {
                        <span class="page-form-error">{i18n::text(&message_error_i18n, key)}</span>
                    })
                }}
            </label>
            {move || {
                submit_status().map(|status| view! {
                    <p class="page-form-status">{status}</p>
                })
            }}
            <button
                class="page-form-submit"
                type="submit"
                disabled=move || submit_state.get() == ContactSubmitState::Sending
            >
                {move || {
                    if submit_state.get() == ContactSubmitState::Sending {
                        view! { <PageLoader /> }.into_any()
                    } else {
                        let submit_label = i18n::text(&submit_i18n, MessageKey::HomepageContactFormSubmit);
                        view! { <span>{submit_label}</span> }.into_any()
                    }
                }}
            </button>
        </form>
    }
}

fn required_error(value: &str, submitted: bool, key: MessageKey) -> Option<MessageKey> {
    (submitted && value.trim().is_empty()).then_some(key)
}

fn is_valid_email(value: &str) -> bool {
    let value = value.trim();
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };

    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}
