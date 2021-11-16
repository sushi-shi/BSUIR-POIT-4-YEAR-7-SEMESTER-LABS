----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:01:47 11/16/2021 
-- Design Name: 
-- Module Name:    tb_device - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
LIBRARY ieee;
USE ieee.std_logic_1164.ALL;
 
-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
USE ieee.numeric_std.ALL;
 
ENTITY tb_device IS
END tb_device;
 
ARCHITECTURE behavior OF tb_device IS 


   --Inputs
   signal a : std_logic := '0';
   signal b : std_logic := '0';
   signal s : std_logic := '0';

   --Outputs
   signal z_beh : std_logic;
  signal z_struct:  std_logic;
  
  signal error: std_logic;
  signal test_vector: std_logic_vector (2 downto 0);
   -- No clocks detected in port list. Replace <clock> below with 
   -- appropriate port name 
 
 constant period : time := 10 ns;
BEGIN
 
  -- Instantiate the Unit Under Test (UUT)
   uut_beh: entity work.DEVICE_1 PORT MAP (
          X => a,
          Y => b,
          Z => s,
          S => z_beh
        );
      
    uut_struct: entity work.DEVICE_1_STRUCT PORT MAP (
          X => a,
          Y => b,
          Z => s,
          S => z_struct
        );

 
 
  a<=test_vector(0);
  b<=test_vector(1);
  s<=test_vector(2);
 
   -- Stimulus process
  stim_proc: process
  begin
  
   for i in 0 to 7 loop
      
        test_vector <= std_logic_vector(to_unsigned(i, test_vector'length)); 
      wait for period;
    end loop;
  
  report "End of simulation" severity failure;
  
  end process;
  
  error<= z_beh xor z_struct;

END;

