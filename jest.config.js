module.exports = {
  preset: "ts-jest",
  testEnvironment: "node",
  transform: {
    "^.+\\.ts?$": ["ts-jest", { tsconfig: "./tsconfig.json" }]
  },
  transformIgnorePatterns: [
    "/node_modules/(?!@zk-email/helpers/)"
  ],  
  // globals: {
  //   'ts-jest': {
  //     tsconfig: './tsconfig.json'
  //   }
  // },
};
