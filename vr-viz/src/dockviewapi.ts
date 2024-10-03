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
                                "type": "leaf",
                                "data": {
                                    "views": [
                                        "gyro"
                                    ],
                                    "activeView": "gyro",
                                    "id": "1"
                                },
                                "size": 620
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
                                        "size": 943
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
                                        "size": 977
                                    }
                                ],
                                "size": 350
                            }
                        ],
                        "size": 1920
                    }
                ],
                "size": 970
            },
            "width": 1920,
            "height": 970,
            "orientation": Orientation.HORIZONTAL
        },
        "panels": {
            "gyro": {
                "id": "gyro",
                "contentComponent": "gyro",
                "title": "Gyroscope"
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
            }
        },
        "activeGroup": "1"
    })
}