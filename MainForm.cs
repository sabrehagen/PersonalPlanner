﻿using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace Planner
{
    public partial class MainForm : Form
    {
        public MainForm()
        {
            InitializeComponent();
            Size drawableSize = new Size(1600, 900);
            this.ClientSize = drawableSize;
            Drawing.SetScreen(drawableSize);
            this.ResizeRedraw = true;

            ColourSchemes.Add("std");
            ColourSchemes.Set("std");
            ColourSchemes.Add("std", "dark", Color.FromArgb(255, 32, 32, 32));
            ColourSchemes.Add("std", "medium", Color.FromArgb(255, 64, 64, 64));
            ColourSchemes.Add("std", "light", Color.FromArgb(255, 128, 128, 128));
            ColourSchemes.Add("std", "green", Color.Green);
            ColourSchemes.Add("std", "red", Color.Red);

            Fonts.Add("medium", 0.05f);
            Fonts.Add("bold", 0.05f, FontStyle.Bold);
            Fonts.Recalculate();

            Scenes.Add("agenda");
            Scenes.Set("agenda");
            
            Label menu = new Label(new Space(0f, 0f, 0.2f, 1f), "dark");
            Label test = new Label(new Space(), "medium");
            uint w = 5, h = 3;
            Grid grid = new Grid(new Space(true), w, h);
            for (int x = 0; x < w; x++)
                for (int y = 0; y < h; y++)
                {
                    Label l = new Label(new Space(), "medium");
                    grid.AddPaddedEven(l, 0.1f, true, (uint)x, (uint)y);
                }
            menu.Add(grid);
            Scenes.Add(menu);
            
            Button button = new Button(new Space(0.3f, 0.1f, 0.4f, 0.1f), () => { Console.WriteLine("hello"); }, "medium", "light", "green");
            Scenes.Add(button);
            string msg = "click me boi";
            TextLine line = new TextLine(new Space(0.3f, 0.1f, 0.4f, 0.1f), msg, "medium", "red");
            Scenes.Add(line);
        }
        
        protected override void OnPaint(PaintEventArgs e)
        {
            base.OnPaint(e);
            Scenes.DrawAll(e.Graphics);
            Drawing.needRedraw = false;
        }

        protected override void OnResize(EventArgs e)
        {
            base.OnResize(e);
            int w = ClientSize.Width;
            float aspect = 16f / 9f;
            this.ClientSize = new Size(w, (int)(w / aspect));
            Drawing.SetScreen(ClientSize);
            Fonts.Recalculate();
        }

        protected override void OnMouseClick(MouseEventArgs e)
        {
            base.OnMouseClick(e);
            MouseEvent ev = new MouseEvent();
            ev.clicked = true;
            ev.down = false;
            ev.button = e.Button;
            ev.x = (float)e.X / ClientSize.Width;
            ev.y = (float)e.Y / ClientSize.Height;
            Scenes.FeedMouseEvent(ev);
            if (Drawing.needRedraw)
                this.Refresh();
        }

        protected override void OnMouseDown(MouseEventArgs e)
        {
            base.OnMouseDown(e);
            MouseEvent ev = new MouseEvent();
            ev.clicked = false;
            ev.down = true;
            ev.button = e.Button;
            ev.x = (float)e.X / ClientSize.Width;
            ev.y = (float)e.Y / ClientSize.Height;
            Scenes.FeedMouseEvent(ev);
            if (Drawing.needRedraw)
                this.Refresh();
        }

        protected override void OnMouseMove(MouseEventArgs e)
        {
            base.OnMouseMove(e);
            MouseEvent ev = new MouseEvent();
            ev.clicked = false;
            ev.button = MouseButtons.None;
            ev.x = (float)e.X / ClientSize.Width;
            ev.y = (float)e.Y / ClientSize.Height;
            Scenes.FeedMouseEvent(ev);
            if (Drawing.needRedraw)
                this.Refresh();
        }
    }
}