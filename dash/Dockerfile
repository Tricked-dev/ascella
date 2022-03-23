FROM node:16-alpine
WORKDIR /usr/server/app
COPY ./package.json ./
RUN yarn install
COPY ./ .
ENV NODE_ENV production
RUN yarn build
CMD yarn start