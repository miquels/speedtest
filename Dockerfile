###--- build UI ---###
FROM node:latest as builder

# make workdir
WORKDIR /app

# copy sources
COPY . .

# compile UI
RUN yarn
# build production files
RUN yarn build


###--- production server ---###
FROM nginx as production

# copy dist files and config
COPY --from=builder /app/dist /usr/share/nginx/html
COPY config.json /usr/share/nginx/html/config.json

# copy nginx config
COPY nginx.conf /etc/nginx/nginx.conf


EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
