'use strict';

const alfy = require('alfy');
const path = require('path');
const fs = require('fs');
const os = require('os');

const historyPath = path.resolve(os.homedir(), '.zsh_history');

fs.readFile(historyPath, 'utf8', (err, data) => {
  const history = data.toString()
    .replace(/\: .*?\:\d;\s?/g, '')
    .split('\n')
    .reverse();

  const matches = alfy
    .inputMatches(history)
    .map(match => ({
      title: match,
      arg: match
    }));

  alfy.output(matches);
});
