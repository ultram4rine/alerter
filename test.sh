#!/bin/sh

curl \
    -H "Content-Type: application/json" \
    --request POST \
    --data '{"receiver":"telegram","status":"firing","alerts":[{"status":"firing","labels":{"alertname":"Fire","severity":"critical"},"annotations":{"message":"Something is on fire"},"startsAt":"2018-11-04T22:43:58.283995108+01:00","endsAt":"2018-11-04T22:46:58.283995108+01:00","generatorURL":"http://localhost:9090/graph?g0.expr=vector%28666%29\u0026g0.tab=1"}],"groupLabels":{"alertname":"Fire"},"commonLabels":{"alertname":"Fire","severity":"critical"},"commonAnnotations":{"message":"Something is on fire"},"externalURL":"http://localhost:9093","version":"4","groupKey":"{}:{alertname=\"Fire\"}"}' \
    localhost:3030
