/*
Auto-generated by: https://github.com/pmndrs/gltfjsx
Command: npx gltfjsx@6.2.16 models/headset.gltf --transform --types 
Files: models/headset.gltf [10.22KB] > /p/10_Rust/convention2024/vr-viz/src/3d/headset-transformed.glb [14.06KB] (-38%)
Author: madha (https://sketchfab.com/madha)
License: CC-BY-4.0 (http://creativecommons.org/licenses/by/4.0/)
Source: https://sketchfab.com/3d-models/vr-headset-366bff6ac3ac4ad1bece6de214b51a95
Title: VR Headset
*/

import * as THREE from 'three'
import React, { useRef } from 'react'
import { useGLTF } from '@react-three/drei'
import { GLTF } from 'three-stdlib'

type GLTFResult = GLTF & {
  nodes: {
    Plane_Fab_0: THREE.Mesh
    Plane_Base_0: THREE.Mesh
    Plane_Glass_0: THREE.Mesh
    BezierCircle_Head_0: THREE.Mesh
  }
  materials: {
    material: THREE.MeshStandardMaterial
    Base: THREE.MeshStandardMaterial
    Glass: THREE.MeshStandardMaterial
    Head: THREE.MeshStandardMaterial
  }
  animations: GLTFAction[]
}

type ContextType = Record<string, React.ForwardRefExoticComponent<JSX.IntrinsicElements['mesh']>>

export function Model(props: JSX.IntrinsicElements['group']) {
  const { nodes, materials } = useGLTF('/headset-transformed.glb') as GLTFResult
  return (
    <group {...props} dispose={null}>
      <mesh geometry={nodes.Plane_Fab_0.geometry} material={materials.material} position={[0, -0.074, 0.686]} rotation={[-1.476, 0.13, 0.001]} scale={0.47} />
      <mesh geometry={nodes.Plane_Base_0.geometry} material={materials.Base} position={[0, -0.074, 0.686]} rotation={[-1.476, 0.13, 0.001]} scale={0.47} />
      <mesh geometry={nodes.Plane_Glass_0.geometry} material={materials.Glass} position={[0, -0.074, 0.686]} rotation={[-1.476, 0.13, 0.001]} scale={0.47} />
      <mesh geometry={nodes.BezierCircle_Head_0.geometry} material={materials.Head} position={[0.004, 0.035, -0.031]} rotation={[-1.476, 0.13, 0.001]} scale={[0.803, 0.919, 0.919]} />
    </group>
  )
}

useGLTF.preload('/headset-transformed.glb')
