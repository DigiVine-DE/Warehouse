package isoxml.utilities;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.Calendar;
import java.util.zip.ZipEntry;
import java.util.zip.ZipInputStream;
import java.util.zip.ZipOutputStream;

import org.apache.log4j.Logger;

/**
 * helper class to import/export, zip/unzip, ... files 
 *
 */
public class ImportExportUtilities {
    private static Logger importExportUtilitiesLogger = Logger.getLogger(ImportExportUtilities.class.getName());
   
    private File tempDir = null;
    private String zipFileImport = null;
    private String zipFileExport = null;
    private String randomDirectoryAndFileName = null;
    private String randomDirectoryImport = null;
    private String randomDirectoryExport = null;
    
   public ImportExportUtilities(Object tempDir) {
        this.tempDir = (File) tempDir;
        createDirectory(getTempDir(), "import");
        createDirectory(getTempDir(), "export");
        Calendar cal = Calendar.getInstance();
        this.randomDirectoryAndFileName = Long.valueOf(cal.getTime().getTime()).toString();
        this.randomDirectoryImport = createDirectory(getImportTempDir(), this.randomDirectoryAndFileName);
        this.randomDirectoryExport = createDirectory(getImportTempDir(), this.randomDirectoryAndFileName);
    }
    
    public String getRandomDirectoryImport() {
        return randomDirectoryImport;
    }

    public String getRandomDirectoryExport() {
        return randomDirectoryExport;
    }

    public String createFilesFromImport(InputStream is) {
        boolean result = false;
        result = createZipFileFromInputStream(is, this.randomDirectoryAndFileName);
        unzip(getImportTempDir(), this.randomDirectoryImport, this.randomDirectoryAndFileName + ".zip");
        if (result == true) {
            return this.randomDirectoryImport;
        } else {
            return null;
        }
    }
    
    public String createZipFilesForExport() {
        this.zipFileExport = zip(this.randomDirectoryExport, getExportTempDir(), this.randomDirectoryAndFileName);
        return this.zipFileExport;
    }
    
    private String createDirectory(String directoryPath, String directoryName) {
        String completeDirectoryName = "";
        File directory = new File(directoryPath, directoryName);
        if (!directory.exists()) {
            directory.mkdir();
        }
        completeDirectoryName = directoryPath + getSeparator() + directoryName;
        return completeDirectoryName;
    }
    
    private String unzip(String inputDirectory, String outputDirectory, String fileName) {
        int read = 0;
        FileOutputStream out = null;
        ZipInputStream in = null;
        byte[] data = new byte[1024];
        ZipEntry entry = null;
        File inputFile = new File(inputDirectory, fileName);
        File outputFile = null;
        File directory = null;

        try {
            in = new ZipInputStream(new FileInputStream(inputFile));
            while ((entry = in.getNextEntry()) != null) {
                if (entry.isDirectory()) {
                    directory = new File(outputDirectory, entry.getName());
                    directory.mkdirs();
                } else {
                    outputFile = new File(outputDirectory, entry.getName());
                    out = new FileOutputStream(outputFile);
                    while ((read = in.read(data, 0, 1024)) != -1) {
                        out.write(data, 0, read);
                    }
                }
                out.close();
            }
            in.close();
        } catch (IOException e) {
            importExportUtilitiesLogger.error("unzip -> unzip not possible." + e.getMessage());
        }
        return outputDirectory + getSeparator();
    }
    
    private String zip(String inputDirectory, String outputDirectory, String fileName) {
        int read = 0;
        FileInputStream in = null;
        ZipOutputStream out = null;
        ZipEntry entry = null;
        File inputFile = null;
        String zipName = fileName;
        if (zipName.indexOf(".zip") == -1) {
            zipName += ".zip";
        }
        File outputFile = new File(outputDirectory, zipName);
        String[] files = new File(inputDirectory).list();
        byte[] data = new byte[1024];

        try {
            if (files.length > 0) {
                out = new ZipOutputStream(new FileOutputStream(outputFile));
                out.setMethod(ZipOutputStream.DEFLATED);
                for (String file : files) {
                    entry = new ZipEntry(file);
                    inputFile = new File(inputDirectory, file);
                    in = new FileInputStream(inputFile);
                    out.putNextEntry(entry);
                    while ((read = in.read(data, 0, 1024)) != -1) {
                        out.write(data, 0, read);
                    }
                    out.closeEntry();
                    in.close();
                }
                out.close();
            } else {
                return null;
            }
        } catch (IOException e) {
            importExportUtilitiesLogger.error("zip -> zip not possible." + e.getMessage());
        }
        return outputDirectory + getSeparator() + zipName;
    }
    
    public boolean createZipFileFromInputStream(InputStream is, String zipFileName) {
        try {
            File f = new File(getImportTempDir() + getSeparator() + zipFileName + ".zip");
            this.zipFileImport = f.getAbsolutePath();
            OutputStream out = new FileOutputStream(f);
            byte buf[] = new byte[1024];
            int len;
            while ((len = is.read(buf)) > 0) {
                out.write(buf, 0, len);
            }
            out.close();
            is.close();
            return true;
        } catch (IOException e) {
            importExportUtilitiesLogger.error("createZipFileFromHttpRequest -> " + e.getMessage());
            return false;
        }
    }
    
    public String getTempDir() {
        if (tempDir == null) {
            importExportUtilitiesLogger.error("using '" + tempDir.getAbsolutePath() + "' as temporary directory");
        }
        return tempDir.getAbsolutePath();
    }
    
    public String getExportTempDir() {
        return getTempDir() + getSeparator() + "export";
    }

    public String getImportTempDir() {
        return getTempDir() + getSeparator() + "import";
    }
    
    public boolean deleteImportFiles() {
        if (this.randomDirectoryImport != null) {
            deleteDirectory(this.randomDirectoryImport);
        }
        if (this.zipFileImport != null) {
            deleteFile(this.zipFileImport);
        }

        return true;
    }
    
    public boolean deleteExportFiles() {
        if (this.randomDirectoryExport != null) {
            deleteDirectory(this.randomDirectoryExport);
        }
        if (this.zipFileExport != null) {
            deleteFile(this.zipFileExport);
        }

        return true;
    }

    private boolean deleteDirectory(String deleteDirectory) {
        boolean success = true;

        if (new File(deleteDirectory).isDirectory()) {
            String[] files = new File(deleteDirectory).list();
            for (String file : files) {
                success = new File(deleteDirectory + getSeparator() + file).delete();
                if (success == false) {
                    return false;
                }
            }
            new File(deleteDirectory).delete();
        }
        return true;
    }

    private boolean deleteFile(String file) {
        return new File(file).delete();
    }
    
    private String getSeparator() {
        return File.separator;
    }
    
    public boolean copy(String inFile, String outFile) {
        try {
            FileInputStream in = new FileInputStream(inFile);
            FileOutputStream out = new FileOutputStream(outFile);

            byte[] b = new byte[1024];
            int bytes = 0;
            int offset = 0;

            while ((bytes = in.read(b)) != -1) {
                out.write(b, offset, bytes);
            }

            in.close();
            out.close();
            return true;

        } catch (FileNotFoundException e) {
            importExportUtilitiesLogger.error(e.getMessage());
        } catch (IOException e) {
            importExportUtilitiesLogger.error(e.getMessage());
        }
        return false;
    }
}
