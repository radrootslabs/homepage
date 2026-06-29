use leptos::{ev::SubmitEvent, prelude::*};
use leptos_lucide_rs::ChevronsUpDown;

use crate::components::{PageLayout, PageSection, PageText};
use crate::i18n::{self, MessageKey};

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
    let submit_label = move || i18n::text(&submit_i18n, MessageKey::HomepageContactFormSubmit);
    let name_error_i18n = i18n.clone();
    let address_error_i18n = i18n.clone();
    let message_error_i18n = i18n.clone();
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
        }
    };
    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        set_submitted.set(true);

        if form_has_errors() {
            return;
        }
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
                        prop:value=move || name.get()
                        on:input=move |event| set_name.set(event_target_value(&event))
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
                        prop:value=move || contact_address.get()
                        on:input=move |event| set_contact_address.set(event_target_value(&event))
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
                        prop:value=move || message.get()
                        on:input=move |event| set_message.set(event_target_value(&event))
                    ></textarea>
                </span>
                {move || {
                    message_error.get().map(|key| view! {
                        <span class="page-form-error">{i18n::text(&message_error_i18n, key)}</span>
                    })
                }}
            </label>
            <button class="page-form-submit" type="submit">
                {submit_label}
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
