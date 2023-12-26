pub const INTRO_BLOCK: &str = r#"
[
    {
        "type": "section",
        "text": {
            "type": "mrkdwn",
            "text": ":wave: <@userid1>, <@userid2>.\n\n It's time to get to know your teammates better! Donut intros people in <#channel> every 2 weeks. \n\n Now that you're here, schedule a time to meet for coffee :coffee:, lunch :sandwich:, or donuts :doughnut:!"
        }
    }
]"#;

pub const MIDPOINT_BLOCK: &str = r#"
[
    {
        "type": "section",
        "text": {
            "type": "mrkdwn",
            "text": "It's time for a midpoint checkin!\n\n *Did you get a chance to meet?*"
        }
    },
    {
        "type": "actions",
        "elements": [
            {
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": "✅ Yes!",
                    "emoji": true
                },
                "value": "yes"
            },
            {
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": "📅 It's scheduled",
                    "emoji": true
                },
                "value": "scheduled"
            },
            {
                "type": "button",
                "text": {
                    "type": "plain_text",
                    "text": "💤 Not yet..",
                    "emoji": true
                },
                "value": "no"
            }
        ]
    }
]"#;