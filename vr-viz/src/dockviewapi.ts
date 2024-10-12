import {AddPanelOptions, DockviewApi, Orientation} from "dockview";
import {toast} from "sonner";

const gyroSettings = {
    "id": "gyro",
    "component": "gyro",
    "title": "Gyroscope",
}

const vrdcSettings = {
    "id": "vrdc",
    "component": "vrdc",
    "title": "VR Distance Configuration",
}

const inferenceSettings = {
    "id": "infr",
    "component": "infr",
    "title": "Inference Settings",
}

const wheelSettings = {
    "id": "whl",
    "component": "whl",
    "title": "Wheel Reading",
}

const servoSettings = {
    "id": "srvo",
    "component": "srvo",
    "title": "Servo Settings",
}

const optimizerSettings = {
    "id": "optm",
    "component": "optm",
    "title": "Optimizer Info",
}

const leaderboardSettings = {
    "id": "ldbd",
    "component": "ldbd",
    "title": "Leaderboard",
}

const utilsSettings = {
    "id": "util",
    "component": "util",
    "title": "Utilities",
}

function openOrCreatePanel(api: DockviewApi, panelDefinition: AddPanelOptions) {
    const panelId = panelDefinition.id;
    const panel = api.getPanel(panelId);

    if (panel === undefined) {
        api.addPanel(panelDefinition);
    } else {
        panel.api.setActive()
    }
}

export function openGyroTab(api: DockviewApi) {
    openOrCreatePanel(api, gyroSettings);
}

export function openVrDistanceConfigurationTab(api: DockviewApi) {
    openOrCreatePanel(api, vrdcSettings);
}

export function openInferenceSettingsTab(api: DockviewApi) {
    openOrCreatePanel(api, inferenceSettings);
}

export function openWheelReadingTab(api: DockviewApi) {
    openOrCreatePanel(api, wheelSettings);
}

export function openServoSettingsTab(api: DockviewApi) {
    openOrCreatePanel(api, servoSettings);
}

export function openOptimizerInfoTab(api: DockviewApi) {
    openOrCreatePanel(api, optimizerSettings);
}

export function openLeaderboardTab(api: DockviewApi) {
    openOrCreatePanel(api, leaderboardSettings);
}

export function openUtilitiesTab(api: DockviewApi) {
    openOrCreatePanel(api, utilsSettings);
}

export function copyToClipboard(api: DockviewApi) {
    const json = api.toJSON();
    navigator.clipboard.writeText(JSON.stringify(json, null, 2)).then(() => console.log("Copied to clipboard"));
    toast.success("Copied to clipboard", {
        richColors: true
    })
}

export function restoreDefaultLayout(api: DockviewApi) {
    api.fromJSON({
        "grid": {
            "root": {
                "type": "branch",
                "data": [
                    {
                        "type": "branch",
                        "data": [
                            {
                                "type": "branch",
                                "data": [
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "gyro"
                                            ],
                                            "activeView": "gyro",
                                            "id": "1"
                                        },
                                        "size": 640
                                    },
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "whl"
                                            ],
                                            "activeView": "whl",
                                            "id": "4"
                                        },
                                        "size": 640
                                    },
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "ldbd"
                                            ],
                                            "activeView": "ldbd",
                                            "id": "7"
                                        },
                                        "size": 640
                                    }
                                ],
                                "size": 589
                            },
                            {
                                "type": "branch",
                                "data": [
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "vrdc"
                                            ],
                                            "activeView": "vrdc",
                                            "id": "2"
                                        },
                                        "size": 384
                                    },
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "srvo"
                                            ],
                                            "activeView": "srvo",
                                            "id": "6"
                                        },
                                        "size": 384
                                    },
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "infr"
                                            ],
                                            "activeView": "infr",
                                            "id": "3"
                                        },
                                        "size": 384
                                    },
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "optm"
                                            ],
                                            "activeView": "optm",
                                            "id": "5"
                                        },
                                        "size": 384
                                    },
                                    {
                                        "type": "leaf",
                                        "data": {
                                            "views": [
                                                "util"
                                            ],
                                            "activeView": "util",
                                            "id": "8"
                                        },
                                        "size": 384
                                    }
                                ],
                                "size": 333
                            }
                        ],
                        "size": 1920
                    }
                ],
                "size": 922
            },
            "width": 1920,
            "height": 922,
            "orientation": Orientation.HORIZONTAL
        },
        "panels": {
            "gyro": {
                "id": "gyro",
                "contentComponent": "gyro",
                "title": "Gyroscope"
            },
            "whl": {
                "id": "whl",
                "contentComponent": "whl",
                "title": "Wheel Reading"
            },
            "ldbd": {
                "id": "ldbd",
                "contentComponent": "ldbd",
                "title": "Leaderboard"
            },
            "vrdc": {
                "id": "vrdc",
                "contentComponent": "vrdc",
                "title": "VR Distance Configuration"
            },
            "srvo": {
                "id": "srvo",
                "contentComponent": "srvo",
                "title": "Servo Settings"
            },
            "infr": {
                "id": "infr",
                "contentComponent": "infr",
                "title": "Inference Settings"
            },
            "optm": {
                "id": "optm",
                "contentComponent": "optm",
                "title": "Optimizer Info"
            },
            "util": {
                "id": "util",
                "contentComponent": "util",
                "title": "Utilities"
            }
        },
        "activeGroup": "8"
    })
}