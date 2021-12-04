----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 05:57:53 PM
-- Design Name: 
-- Module Name: tb_task2_regfile - Behavioral
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
-- Description: 
-- 
-- Dependencies: 
-- 
-- Revision:
-- Revision 0.01 - File Created
-- Additional Comments:
-- 
----------------------------------------------------------------------------------


library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity tb_task2_regfile is
--  Port ( );
end tb_task2_regfile;

architecture Behavioral of tb_task2_regfile is
    constant period: time := 50 ns;
    constant INITREG: STD_LOGIC_VECTOR := "0000";
    constant p: integer := 2;
    
    signal INIT: STD_LOGIC := '0';
    signal WDP: STD_LOGIC_VECTOR(INITREG'range) := (others => '0');
    signal WA: STD_LOGIC_VECTOR(p - 1 downto 0) := (others => '0');
    signal CLK: STD_LOGIC := '0';
    signal RA: STD_LOGIC_VECTOR(p - 1 downto 0) := (others => '0');
    
    signal RDP: STD_LOGIC_VECTOR(INITREG'range);
    
begin
    uut_1: entity work.REG_FILE 
		generic map(INITREG, p) 
      port map (INIT, WDP, WA, CLK, RA, RDP);
    
    CLK_proc: process
    begin
        CLK <= '0';
        wait for period / 2;
        CLK <= '1';
        wait for period / 2;
    end process;
    
    test_proc: process
    begin
        WDP <= "1010";
        WA <= "00";
        wait for period * 2;
		  
        WDP <= "0101";
		  wait for period * 2;
		  
        RA <= "00";
        wait for period * 2;
        
        WDP <= "1111";
        WA <= "10";
        wait for period * 2;
        
        RA <= "10";
        wait for period * 2;
        wait;
    end process;
end Behavioral;
