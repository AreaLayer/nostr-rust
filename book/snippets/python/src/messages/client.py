from typing import cast
from nostr_sdk import Keys, EventBuilder, ClientMessage, Filter, ClientMessageEnum


def client_message():
    keys = Keys.generate()
    event = EventBuilder.text_note("TestTextNoTe").sign_with_keys(keys)

    print()
    print("Client Messages:")

    # ANCHOR: event-message
    # Event client message
    print("  Event Client Message:")
    message = ClientMessage.event(event)
    print(f"     - Event Message: {message.as_enum().is_event_msg()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: event-message

    print()
    # ANCHOR: req-message
    # Request client message
    print("  Request Client Message:")
    f = Filter().id(event.id())
    message = ClientMessage.req(subscription_id="ABC123", filter=f)
    print(f"     - Request Message: {message.as_enum().is_req()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: req-message

    print()
    # ANCHOR: close-message
    # Close client message
    print("  Close Client Message:")
    message = ClientMessage.close("ABC123")
    print(f"     - Close Message: {message.as_enum().is_close()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: close-message

    print()
    # ANCHOR: parse-message
    # Parse Messages from JSON and/or Enum
    print("  Parse Client Messages:")
    message = ClientMessage.from_json('["REQ","ABC123",{"#p":["421a4dd67be773903f805bcb7975b4d3377893e0e09d7563b8972ee41031f551"]}]')
    print(f"     - ENUM: {message.as_enum()}")
    f = Filter().pubkey(keys.public_key())
    message = ClientMessage.from_enum(cast(ClientMessageEnum, ClientMessageEnum.REQ("ABC123", filter=f)))
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: parse-message

    print()
    # ANCHOR: auth-message
    # Auth client message  (NIP42)
    print("  Auth Client Message:")
    message = ClientMessage.auth(event)
    print(f"     - Auth Message: {message.as_enum().is_auth()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: auth-message

    print()
    # ANCHOR: count-message
    # Count client message (NIP45)
    print("  Count Client Message:")
    f = Filter().pubkey(keys.public_key())
    message = ClientMessage.count(subscription_id="ABC123", filter=f)
    print(f"     - Count Message: {message.as_enum().is_count()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: count-message

    print()
    # ANCHOR: neg-open
    # Negative Open Message
    print("  Negative Client Message (open):")
    message = ClientMessage.from_enum(cast(ClientMessageEnum, ClientMessageEnum.NEG_OPEN("ABC123", filter=f, id_size=32, initial_message="<hex-msg>")))
    print(f"     - Negative Error Open: {message.as_enum().is_neg_open()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: neg-open

    print()
    # ANCHOR: neg-close
    # Negative Close Message
    print("  Negative Client Message (close):")
    message = ClientMessage.from_enum(cast(ClientMessageEnum, ClientMessageEnum.NEG_CLOSE("ABC123")))
    print(f"     - Negative Error Close: {message.as_enum().is_neg_close()}")
    print(f"     - JSON: {message.as_json()}")
    # ANCHOR_END: neg-close

    print()
    # ANCHOR: neg-msg
    # Negative Error Message
    print("  Negative Client Message (message):")
    enum_msg = ClientMessageEnum.NEG_MSG("ABC123", message="This is not the message you are looking for")
    message = ClientMessage.from_enum(cast(ClientMessageEnum, enum_msg))
    print(f"     - JSON: {message.as_json()}")
    print(f"     - Negative Error Message: {message.as_enum().is_neg_msg()}")
    # ANCHOR_END: neg-msg

if __name__ == '__main__':
   client_message()