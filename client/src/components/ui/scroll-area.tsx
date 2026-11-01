import * as React from "react";
import * as ScrollAreaPrimitive from "@radix-ui/react-scroll-area";
import { cn } from "@/lib/utils";

const ScrollArea = React.forwardRef<
  React.ElementRef<typeof ScrollAreaPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof ScrollAreaPrimitive.Root>
>(({ className, children, ...props }, ref) => (
  <ScrollAreaPrimitive.Root
    ref={ref}
    className={cn("relative overflow-auto", className)}
    {...props}
  >
    <ScrollAreaPrimitive.Viewport className="h-full w-full rounded-[inherit]">
      {children}
    </ScrollAreaPrimitive.Viewport>
    <ScrollAreaPrimitive.Scrollbar
      className="flex select-none touch-none p-0.5 bg-gray-700/50 hover:bg-gray-700/80 transition-colors duration-200"
      orientation="vertical"
      style={{ width: "10px" }}
    >
      <ScrollAreaPrimitive.Thumb className="flex-1 rounded-full bg-white/50" />
    </ScrollAreaPrimitive.Scrollbar>
    <ScrollAreaPrimitive.Scrollbar
      className="flex select-none touch-none p-0.5 bg-gray-700/50 hover:bg-gray-700/80 transition-colors duration-200"
      orientation="horizontal"
      style={{ height: "10px" }}
    >
      <ScrollAreaPrimitive.Thumb className="flex-1 rounded-full bg-white/50" />
    </ScrollAreaPrimitive.Scrollbar>
    <ScrollAreaPrimitive.Corner />
  </ScrollAreaPrimitive.Root>
));
ScrollArea.displayName = ScrollAreaPrimitive.Root.displayName;

export { ScrollArea };