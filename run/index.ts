"use strict";

import path from "path";

import * as core from "@actions/core";
import * as exec from "@actions/exec";
import * as tc from "@actions/tool-cache";

const {
	name: ACTION_NAME,
	repository: REPOSITORY,
	version: VERSION,
} = require("../package.json");

const COMMAND = path.basename(path.dirname(require.main!.filename));

const { RUNNER_TEMP } = process.env;
const { platform: PLATFORM } = process;

const NAME = ACTION_NAME.slice(ACTION_NAME.lastIndexOf("/") + 1);
const BASE_URL = `${REPOSITORY}/releases/download/${VERSION}`;
const FILE_PREFIX = `${NAME}-v${VERSION}`;

async function downloadLinux(): Promise<string> {
	const file = `${FILE_PREFIX}-linux-x64.tar.gz`;
	const url = `${BASE_URL}/${file}`;
	const downloadPath = await tc.downloadTool(url);
	const extractPath = await tc.extractTar(downloadPath, RUNNER_TEMP);
	const extractedFile = path.join(extractPath, NAME);

	return tc.cacheFile(extractedFile, NAME, ACTION_NAME, VERSION);
}

async function linux(): Promise<void> {
	const cacheDir = tc.find(ACTION_NAME, "0.0.0") || await downloadLinux();
	const binary = path.join(cacheDir, NAME);

	await exec.exec(binary, [COMMAND]);
}

async function run(): Promise<void> {
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
