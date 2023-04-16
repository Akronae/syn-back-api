# installing depencies and building app
FROM nimlang/nim as builder

WORKDIR /usr/src/app

COPY . .

RUN nimble refresh
RUN nimble install
RUN nimble release


# only keeping executable and running app
FROM nimlang/nim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/main ./main
RUN chmod +x ./main

CMD [ "./main" ]