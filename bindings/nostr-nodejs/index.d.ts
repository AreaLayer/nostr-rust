/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsEventBuilder = EventBuilder
export class EventBuilder {
  constructor(kind: bigint, content: string, tags: Array<Array<string>>)
  toEvent(keys: JsKeys): JsEvent
  toPowEvent(keys: JsKeys, difficulty: number): JsEvent
  static setMetadata(metadata: JsMetadata): this
  static addRecommendedRelay(url: string): this
  static newTextNote(content: string, tags: Array<Array<string>>): this
  static setContactList(list: Array<JsContact>): EventBuilder
  static newEncryptedDirectMsg(senderKeys: JsKeys, receiverPubkey: JsPublicKey, content: string): this
  static repost(eventId: JsEventId, publicKey: JsPublicKey): EventBuilder
  static delete(ids: Array<JsEventId>, reason?: string | undefined | null): EventBuilder
  static newReaction(eventId: JsEventId, publicKey: JsPublicKey, content: string): EventBuilder
  static newChannel(metadata: JsMetadata): this
  static setChannelMetadata(channelId: string, relayUrl: string | undefined | null, metadata: JsMetadata): this
  static newChannelMsg(channelId: string, relayUrl: string | undefined | null, content: string): this
  static hideChannelMsg(messageId: JsEventId, reason?: string | undefined | null): EventBuilder
  static muteChannelUser(pubkey: JsPublicKey, reason?: string | undefined | null): EventBuilder
  static auth(challenge: string, relay: string): this
}
export type JsEventId = EventId
export class EventId {
  constructor(pubkey: JsPublicKey, createdAt: bigint, kind: bigint, tags: Array<Array<string>>, content: string)
  static fromSlice(bytes: Array<number>): JsEventId
  static fromHex(hex: string): JsEventId
  static fromBech32(id: string): JsEventId
  asBytes(): Array<number>
  toHex(): string
  toBech32(): string
}
export type JsEvent = Event
export class Event {
  get id(): EventId
  get pubkey(): JsPublicKey
  get createdAt(): bigint
  get kind(): bigint
  get tags(): Array<Array<string>>
  get content(): string
  get signature(): string
  verify(): boolean
  static fromJson(json: string): JsEvent
  asJson(): string
}
export type JsPublicKey = PublicKey
export class PublicKey {
  static fromHex(hex: string): JsPublicKey
  static fromBech32(pk: string): JsPublicKey
  toHex(): string
  toBech32(): string
}
export type JsSecretKey = SecretKey
export class SecretKey {
  static fromHex(hex: string): JsSecretKey
  static fromBech32(sk: string): JsSecretKey
  toHex(): string
  toBech32(): string
}
export type JsKeys = Keys
export class Keys {
  constructor(secretKey: SecretKey)
  static fromPublicKey(publicKey: PublicKey): JsKeys
  static fromSkStr(secretKey: string): JsKeys
  static fromPkStr(publicKey: string): JsKeys
  static generate(): JsKeys
  static fromMnemonic(mnemonic: string, passphrase?: string | undefined | null): JsKeys
  publicKey(): PublicKey
  secretKey(): SecretKey
}
export type JsContact = Contact
export class Contact {
  constructor(publicKey: PublicKey, relayUrl?: string | undefined | null, alias?: string | undefined | null)
}
export type JsMetadata = Metadata
export class Metadata {
  constructor()
  static fromJson(json: string): JsMetadata
  asJson(): string
  name(name: string): Metadata
  displayName(displayName: string): Metadata
  about(about: string): Metadata
  website(url: string): this
  picture(url: string): this
  banner(url: string): this
  nip05(nip05: string): Metadata
  lud06(lud06: string): Metadata
  lud16(lud16: string): Metadata
}
