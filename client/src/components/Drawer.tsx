import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
} from "@/components/ui/drawer";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";

import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { isNumber } from "@/lib/utils";

const formSchema = z.object({
  user_id: z.string().refine((data) => {
    if (isNumber(data)) {
      return false;
    }
    return true;
  }, "user_id must be a number"),
  password: z.string().min(8, {
    message: "password must be greater than 7",
  }),
});

export default function DrawerComponent() {
  const [drawerOpen, setDrawerOpen] = useState(false);
  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    const drawer = searchParams.get("drawer");
    setDrawerOpen(drawer === "open");
  }, [drawerOpen]);

  // Form Start
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      user_id: "",
      password: "",
    },
  });
  function onSubmit(values: z.infer<typeof formSchema>) {
    console.log(values);
  }
  // Form End
  return (
    <Drawer open={drawerOpen}>
      <DrawerContent className="w-[95%] mx-auto max-w-[475px] bg-slate-200">
        <DrawerHeader>
          <DrawerTitle className="text-2xl font-bold">
            Welcome to Chatician!
          </DrawerTitle>
          <DrawerDescription>
            This is a demo app, so you can enter anything you want.
          </DrawerDescription>
        </DrawerHeader>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="p-4 flex flex-col gap-y-2"
          >
            <FormField
              control={form.control}
              name="user_id"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>User ID</FormLabel>
                  <FormControl>
                    <Input
                      className="w-full p-2 border rounded-md"
                      placeholder="Enter your user_id"
                      type="text"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input
                      className="w-full p-2 border rounded-md"
                      placeholder="Enter your user_id"
                      type="text"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <Button type="submit">Dive In</Button>
          </form>
        </Form>

        <DrawerFooter>
          <DrawerClose onClick={() => {}}>
            <span className="font-semibold text-sm text-blue-600">Cancel</span>
          </DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  );
}
