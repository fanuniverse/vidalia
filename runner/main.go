package runner

import (
    "log"
    "time"
    "github.com/streadway/amqp"
    "github.com/buger/jsonparser"
    "encoding/json"
    "vidalia/image"
    "vidalia/config"
)

func ObtainChannelConnection() (*amqp.Connection, *amqp.Channel) {
    conn, err := amqp.Dial(config.AmqpUri)
    for err != nil {
        log.Printf("Cannot reach the AMQP broker. Retrying in 5 seconds...")

        time.Sleep(5 * time.Second)
        conn, err = amqp.Dial(config.AmqpUri)
    }

    ch, err := conn.Channel()
    if err != nil { log.Fatal(err) }

    log.Printf("Vidalia is ready to consume processing requests.")

    return conn, ch
}

func RunService(ch *amqp.Channel) {
    messages, _ := ch.Consume(
        "image.process",
        "",
        false, /* auto ack */
        false, /* exclusive */
        false,
        false, /* no wait */
        nil)

    for m := range messages {
        processed, err := process(m.Body)
        if err != nil {
            log.Printf("Discarding %s, reason: %s\n", m.Body, err)
            m.Reject(false)
        } else {
            reply, err := encodeImage(processed)
            if err != nil {
                log.Printf("Discarding %s, reason: %s\n", m.Body, err)
                m.Reject(false)
            } else {
                _ = ch.Publish(
                    "",
                    "image.processed",
                    false, /* mandatory */
                    false, /* immediate */
                    reply)

                m.Ack(false)
            }
        }
    }
}

func process(message []byte) (*image.Image, error) {
    id, err := jsonparser.GetString(message, "id")
    cachedFile, err := jsonparser.GetString(message, "file")
    if err != nil { return nil, err }

    img, err := image.NewImage(cachedFile, id)
    if err != nil { return nil, err }

    err = img.Process()
    if err != nil { return nil, err }

    return img, nil
}

func encodeImage(img *image.Image) (msg amqp.Publishing, err error) {
    body, err := json.Marshal(img)
    if err != nil { return msg, err }

    return amqp.Publishing {
        DeliveryMode:    amqp.Persistent,
        Timestamp:       time.Now(),
        ContentType:     "application/json",
        Body:            body,
    }, nil
}
