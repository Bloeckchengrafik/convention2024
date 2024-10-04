<div style="text-align: center"><img src="./ftvr.svg"></div>

# ftVR - ftConvention2024 Project

> **What the heck is ftConvention?**
> The fischertechnik Convention is a yearly event where fischertechnik enthusiasts from all over europe meet to share
> their knowledge and experience with fischertechnik as well as show off their current projects. The event is organized
> by the [ftcommunity](https://ftcommunity.de) and is open to everyone who is interested in fischertechnik. The
> fischertechnik
> Convention is a great opportunity to meet other fischertechnik enthusiasts, learn new things, and have fun with
> tech. All under the motto "Technik spielend begreifen" (Understanding technology through play).

My friends and I always build something new each year to show off at ftConvention.
This year we decided to build a VR headset using fischertechnik parts. We call it ftVR.
If you're thinking "Why would you build a VR headset out of fischertechnik parts?" then you're asking the wrong
question.
It's not about why, it's about why not! It works and it's fun to build. That's all that matters (for us at least).

## What can it do?

ftVR is a VR headset that connects to a big parkour with a robot inside. The robot is controlled by the player using the
VR headset and a steering wheel/pedal set. The player can see what the robot sees and control it to navigate through the
parkour. The goal is to get to the exi as fast as possible but that's easier said than done.

We currently have the following components in our setup:

- VR headset (Screen, lenses, head strap, 2 cameras and gyroscope)
- Robot (ftSwarm, 2 cameras, (servo) motors...)
- Parkour (ftSwarm, a LOT of fischertechnik parts)
- Steering wheel/pedal set (ftSwarm, ftKnob)

The VR headset projects the users hand onto the screen over the robot's camera feed. This is done to make it easier for
the player to control the robot.

## How does it work?

There are different software components that make up the ftVR system:

- vrcore (🚀 Rust 🚀)
- ftVR wizard (Typescript/React)
- SwarmControl (C++)
- CameraControl (C++)

The `vrcore` is the main component that connects all the other components. It communicates with the VR headset, the
robot, the parkour and the steering wheel/pedal set. It also handles the camera feed and the hand tracking.

The `ftVR wizard` is a web app that allows the user to configure the ftVR system. It's used to set up and configure
different parts of the system like the eye distance in the headset or the control scheme for the robot.

The `SwarmControl` is the software that runs on the robot and parkour. It handles the communication with the `vrcore`.

The `CameraControl` is the software that runs on the cameras. It handles the camera feed and publishes it as a mjpeg
stream.

### How is Hand masking done?

Well glad you asked! (I know you didn't but I'm going to tell you anyways)

The hand masking uses the ONNX model export of either YoloV8 or the 🔥 cutting edge 🔥 Yolo11 for image
segmentation.
The steering wheel is detected as a clock (lol what?) and can be seperated like that. Before the final mask is applied,
I blur the mask.

To run the YOLO Model, I use a custom Rust implementation based on Ultralytics' example for Rust. But their code if
terribly slow:
Their preprocessing takes around 100ms per image (so 200ms for me, VR is an absolute pain), for me it takes around 8ms
for both.
Postprocessing is also around 3x improved but I couldn't be bothered optimize it further.

**Important:** Many of the used optimizations are only possible under specific assumptions about I/O and
the machine. For example, I don't export any mask that I don't need to. This is only possible because I know that I only
need the mask for the hand and the steering wheel. If you want to use this code for your own project, you will have to
make sure that the optimizations are still valid for your use case.

To get my lovely 20fps (almost all I can get using my cameras, ESP32 Xiao Sense) I don't run inference on every frame
but on every third frame.
This is an acceptable amount of delay for me and I don't want to spend a lot more one (or two) better GPUs or TPUs.
Right now I'm rocking a RTX 2060 Super and two Coral AI TPUs, but the latter ones don't have enough data transfer speed 
to be useful for this project.

## How can I try it out?

**Please don't attempt to use code from this project if you don't want to have a seizure.**
This code is put together in a hurry and is not meant to be used by anyone other than me, please excuse the mess.

But if you're early enough and live near me, you can come to the ftConvention2024 and try it out for yourself.
If you ask nicely I might even let you play around with the settings, but that depends on how busy I am :)

![img.png](ftConvention2024.png)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Although not required, I would love to hear from you if you use this project in any way.
Just shoot me an email at christian.bergschneider \[slinky-a-thing\] gmx.de or a Message via Discord (bloeckchengrafik)