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
                                        "size": 960
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
                                        "size": 960
                                    }
                                ],
                                "size": 601
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
                                        "size": 480
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
                                        "size": 480
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
                                        "size": 480
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
                                        "size": 480
                                    }
                                ],
                                "size": 339
                            }
                        ],
                        "size": 1920
                    }
                ],
                "size": 940
            },
            "width": 1920,
            "height": 940,
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
            "vrdc": {
                "id": "vrdc",
                "contentComponent": "vrdc",
                "title": "VR Distance Configuration"
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
            "srvo": {
                "id": "srvo",
                "contentComponent": "srvo",
                "title": "Servo Settings"
            }
        },
        "activeGroup": "4"
    })
}