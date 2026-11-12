package isoxml.controller;

import isoxml.exception.FileException;
import isoxml.exception.ManagerException;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;


/**
 * ManagerInterface for all implemented Managers like TaskManager
 *
 */
public interface ManagerInterface {
 	void init(java.util.Properties properties, Object tmpDir)throws ManagerException;

    void processRequest(HttpServletRequest request, HttpServletResponse response, String prefix)    
    throws java.io.IOException, ManagerException, FileException;
   
    String getPackageClassName();
}
