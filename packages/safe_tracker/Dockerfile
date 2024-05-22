# Use an official Node.js runtime as a parent image
FROM node:16

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json (or yarn.lock) to leverage Docker cache
COPY package*.json ./
COPY yarn.lock ./

# Install any dependencies
RUN npm install
# For yarn, use the following command instead
# RUN yarn install

# Copy the rest of your app's source code from your host to your image filesystem.
COPY . .

# Compile TypeScript to JavaScript
RUN npx tsc

# Your application's default port, if it has one
EXPOSE 3000

# Run the compiled app
CMD [ "node", "dist/safe_tracker.js" ]