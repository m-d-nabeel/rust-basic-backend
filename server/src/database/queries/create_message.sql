SELECT *
FROM chatician.channel_messages
         JOIN chatician.messages c on c.id = channel_messages.message_id
WHERE channel_id = 1;