"use strict";

const path = require("path");

const core = require("@actions/core");
const exec = require("@actions/exec");
const tc = require("@actions/tool-cache");

// TODO: Can these be grabbed from the running action?
const ACTION_NAME = "@kjvalencik/actions";
const REPOSITORY = "https://github.com/kjvalencik/actions";
const VERSION = "0";
const COMMAND = path.basename(require.main.path);

const { RUNNER_TEMP } = process.env;
const { platform: PLATFORM } = process;

const NAME = ACTION_NAME.slice(ACTION_NAME.lastIndexOf("/") + 1);
const BASE_URL = `${REPOSITORY}/releases/download/${VERSION}`;
const FILE_PREFIX = `${NAME}-v${VERSION}`;

async function downloadLinux() {
	const file = `${FILE_PREFIX}-linux-x64.tar.gz`;
	const url = `${BASE_URL}/${file}`;
	const downloadPath = await tc.downloadTool(url);
	const extractPath = await tc.extractTar(downloadPath, RUNNER_TEMP);
	const extractedFile = path.join(extractPath, NAME);

	return tc.cacheFile(extractedFile, NAME, ACTION_NAME, VERSION);
}

async function linux() {
	const cacheDir = tc.find(ACTION_NAME, '0.0.0') || await downloadLinux();
	const binary = path.join(cacheDir, NAME);

	return exec.exec(binary, [COMMAND]);
}

async function run() {
	switch (PLATFORM) {
		case "linux": return linux();
		case "darwin":
		case "win32":
		case "aix":
		case "freebsd":
		case "openbsd":
		case "sunos":
		default:
			throw new Error(`Unsupported platform: ${PLATFORM}`);
	}
}

run().catch((error) => core.setFailed(error.message));
