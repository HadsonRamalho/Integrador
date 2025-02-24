/* eslint-disable @typescript-eslint/no-explicit-any */
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import Layout from "@/layouts/default";
import { useState } from "react";

export default function UpdateMachine() {
  const [name, setName] = useState("");
  const [serialNumber, setSerialNumber] = useState("");
  const [rentValue, setRentValue] = useState(0);
  const [rentDisponibility, setRentDisponibility] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("");
  const [machineImages, setMachineImages] = useState<(File | null)[]>([]);
  const imageIds: string[] = [];
  const [loading, setIsLoading] = useState(false);

  return (
  <Layout>
    
    </Layout>
  );
}