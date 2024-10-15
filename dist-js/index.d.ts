interface SendNotificationRequest {
    title: String;
    body?: String;
    bigTitle?: String;
    bigDescription?: String;
    bigSummaryText?: String;
}
export declare function send_notification(value: SendNotificationRequest): Promise<string | null>;
export {};
